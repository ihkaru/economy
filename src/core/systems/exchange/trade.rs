use std::collections::BTreeMap;
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::EconomicActor;
use crate::core::domain::item::id::{ItemId, ItemInstanceId};
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::time::{RunId, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::statistic_store::StatisticStorePort;
use crate::core::systems::exchange::market_intelligence::{
    agent_has_market_access, derive_market_scarcity_multiplier, evaluate_marginal_utility,
};

/// Executes bilateral knowledge apprenticeship or physical commodity barter between two reachable agents
pub fn perform_trade_and_services(
    run_id: &RunId,
    current_tick: Tick,
    agent_a_id: AgentId,
    agent_b_id: AgentId,
    agent_store: &mut dyn AgentStorePort,
    ledger_store: &mut dyn LedgerStorePort,
    stat_store: &dyn StatisticStorePort,
    next_trx_id: &mut u64,
    next_instance_id: &mut u64,
) {
    let (a_cal, a_inv) = agent_store
        .get_human(agent_a_id)
        .map(|a| (a.calorie_reserve, a.inventory.clone()))
        .unwrap_or((0.0, BTreeMap::new()));
    let (b_cal, b_inv) = agent_store
        .get_human(agent_b_id)
        .map(|b| (b.calorie_reserve, b.inventory.clone()))
        .unwrap_or((0.0, BTreeMap::new()));

    // Case A: Knowledge Service Trade (Apprenticeship/Education)
    let mut knowledge_trade_occurred = false;
    for k_id in [
        ItemId::KNOWLEDGE_RAFT_BUILDING,
        ItemId::KNOWLEDGE_FISH_CURING,
        ItemId::KNOWLEDGE_TOOL_CRAFTING,
    ] {
        if a_inv.contains_key(&k_id) && !b_inv.contains_key(&k_id) {
            let b_food = [ItemId::GRAIN, ItemId::FISH, ItemId::BERRIES]
                .into_iter()
                .find(|f| b_inv.get(f).copied().unwrap_or(0) >= 2);

            if let Some(payment_food) = b_food {
                if let Some(agent_a) = agent_store.get_human_mut(agent_a_id) {
                    agent_a.add_item(payment_food, 2);
                }
                if let Some(agent_b) = agent_store.get_human_mut(agent_b_id) {
                    let _ = agent_b.remove_item(payment_food, 2);
                    agent_b.add_item(k_id, 1);
                }

                let inst_k = ItemInstance::new(
                    k_id,
                    1,
                    serde_json::json!({"nature": "NonRivalKnowledge"}),
                ).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;

                let inst_food = ItemInstance::new(
                    payment_food,
                    2,
                    serde_json::json!({"nature": "RivalPhysical"}),
                ).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;

                let entry = LedgerEntry::new(
                    *next_trx_id,
                    run_id.clone(),
                    current_tick,
                    agent_a_id,
                    agent_b_id,
                    vec![inst_k],
                    vec![inst_food],
                    serde_json::json!({
                        "transaction_type": "knowledge_service_trade",
                        "knowledge_item_id": k_id.0,
                        "tuition_paid_item_id": payment_food.0,
                        "tuition_paid_qty": 2,
                    }),
                );
                *next_trx_id += 1;
                let _ = ledger_store.record(entry);

                knowledge_trade_occurred = true;
                break;
            }
        }
    }

    // Case B: Bilateral Physical Goods Barter (if no knowledge trade occurred)
    if !knowledge_trade_occurred {
        let a_informed = agent_has_market_access(agent_a_id, agent_store);
        let b_informed = agent_has_market_access(agent_b_id, agent_store);

        let mult_a = |id: ItemId| if a_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };
        let mult_b = |id: ItemId| if b_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };

        let a_offer = a_inv
            .iter()
            .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
            .min_by(|(id1, q1), (id2, q2)| {
                let u1 = evaluate_marginal_utility(**id1, a_cal, **q1) * mult_a(**id1);
                let u2 = evaluate_marginal_utility(**id2, a_cal, **q2) * mult_a(**id2);
                u1.partial_cmp(&u2).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| *id);

        let b_offer = b_inv
            .iter()
            .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
            .min_by(|(id1, q1), (id2, q2)| {
                let u1 = evaluate_marginal_utility(**id1, b_cal, **q1) * mult_b(**id1);
                let u2 = evaluate_marginal_utility(**id2, b_cal, **q2) * mult_b(**id2);
                u1.partial_cmp(&u2).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| *id);

        if let (Some(item_a), Some(item_b)) = (a_offer, b_offer)
            && item_a != item_b
        {
            let a_stock_a = a_inv.get(&item_a).copied().unwrap_or(0);
            let a_stock_b = a_inv.get(&item_b).copied().unwrap_or(0);
            let b_stock_a = b_inv.get(&item_a).copied().unwrap_or(0);
            let b_stock_b = b_inv.get(&item_b).copied().unwrap_or(0);

            let u_a_gives = evaluate_marginal_utility(item_a, a_cal, a_stock_a) * mult_a(item_a);
            let u_a_receives = evaluate_marginal_utility(item_b, a_cal, a_stock_b) * mult_a(item_b);
            let u_b_gives = evaluate_marginal_utility(item_b, b_cal, b_stock_b) * mult_b(item_b);
            let u_b_receives = evaluate_marginal_utility(item_a, b_cal, b_stock_a) * mult_b(item_a);

            if u_a_receives > u_a_gives && u_b_receives > u_b_gives {
                if let Some(agent_a) = agent_store.get_human_mut(agent_a_id) {
                    let _ = agent_a.remove_item(item_a, 1);
                    agent_a.add_item(item_b, 1);
                }
                if let Some(agent_b) = agent_store.get_human_mut(agent_b_id) {
                    let _ = agent_b.remove_item(item_b, 1);
                    agent_b.add_item(item_a, 1);
                }

                let instance_a = ItemInstance::new(
                    item_a,
                    1,
                    serde_json::json!({"source_agent": agent_a_id.0}),
                ).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;

                let instance_b = ItemInstance::new(
                    item_b,
                    1,
                    serde_json::json!({"source_agent": agent_b_id.0}),
                ).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;

                let entry = LedgerEntry::new(
                    *next_trx_id,
                    run_id.clone(),
                    current_tick,
                    agent_a_id,
                    agent_b_id,
                    vec![instance_a],
                    vec![instance_b],
                    serde_json::json!({
                        "transaction_type": "bilateral_barter",
                        "item_a_id": item_a.0,
                        "item_b_id": item_b.0,
                        "surplus_a": u_a_receives - u_a_gives,
                        "surplus_b": u_b_receives - u_b_gives,
                        "agent_a_informed": a_informed,
                        "agent_b_informed": b_informed,
                        "information_asymmetry": a_informed != b_informed,
                    }),
                );
                *next_trx_id += 1;
                let _ = ledger_store.record(entry);
            }
        }
    }
}

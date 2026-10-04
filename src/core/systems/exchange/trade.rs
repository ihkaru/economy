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

    // Case A: Medical Caregiving Service (Emergency Healthcare)
    let mut medical_service_occurred = false;
    let a_sick = agent_store.get_human(agent_a_id).map(|a| a.is_sick()).unwrap_or(false);
    let b_sick = agent_store.get_human(agent_b_id).map(|b| b.is_sick()).unwrap_or(false);

    let medical_pair = if b_sick && !a_sick && (a_inv.contains_key(&ItemId::KNOWLEDGE_HERBAL_MEDICINE) || a_inv.contains_key(&ItemId::SERVICE_MEDICAL)) {
        Some((agent_a_id, agent_b_id, &b_inv))
    } else if a_sick && !b_sick && (b_inv.contains_key(&ItemId::KNOWLEDGE_HERBAL_MEDICINE) || b_inv.contains_key(&ItemId::SERVICE_MEDICAL)) {
        Some((agent_b_id, agent_a_id, &a_inv))
    } else {
        None
    };

    if let Some((healer_id, patient_id, patient_inv)) = medical_pair {
        let fee_opt = [ItemId::GRAIN, ItemId::FISH, ItemId::BERRIES, ItemId::TIMBER]
            .into_iter()
            .find(|f| patient_inv.get(f).copied().unwrap_or(0) >= 1);

        if let Some(fee_item) = fee_opt {
            if let Some(h) = agent_store.get_human_mut(healer_id) { h.add_item(fee_item, 1); }
            if let Some(p) = agent_store.get_human_mut(patient_id) {
                let _ = p.remove_item(fee_item, 1);
                p.set_sick(false);
            }
            let inst_service = ItemInstance::new(ItemId::SERVICE_MEDICAL, 1, serde_json::json!({"nature": "IntangibleHealthcareService"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
            *next_instance_id += 1;
            let inst_fee = ItemInstance::new(fee_item, 1, serde_json::json!({"nature": "RivalPhysical"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
            *next_instance_id += 1;
            let entry = LedgerEntry::new(*next_trx_id, run_id.clone(), current_tick, healer_id, patient_id, vec![inst_service], vec![inst_fee], serde_json::json!({
                "transaction_type": "medical_care_service", "patient_id": patient_id.0, "healer_id": healer_id.0, "service_rendered": "Herbal Treatment & Healing", "fee_paid_item_id": fee_item.0
            }));
            *next_trx_id += 1;
            let _ = ledger_store.record(entry);
            medical_service_occurred = true;
        }
    }

    // Case B: Knowledge Service Trade (Apprenticeship/Education)
    let mut knowledge_trade_occurred = false;
    if !medical_service_occurred {
        let all_knowledges = [
            ItemId::KNOWLEDGE_FIRE_MAKING, ItemId::KNOWLEDGE_TOOL_CRAFTING,
            ItemId::KNOWLEDGE_BASKET_WEAVING, ItemId::KNOWLEDGE_HERBAL_MEDICINE,
            ItemId::KNOWLEDGE_POTTERY_MAKING, ItemId::KNOWLEDGE_LEATHER_WORKING,
            ItemId::KNOWLEDGE_RAFT_BUILDING, ItemId::KNOWLEDGE_FISH_CURING,
        ];
        let tuition_foods = [
            ItemId::GRAIN, ItemId::FISH, ItemId::BERRIES, ItemId::RAW_MEAT,
            ItemId::CURED_MEAT, ItemId::CURED_FISH, ItemId::SMOKED_MEAT, ItemId::SMOKED_FISH,
        ];

        let teach_directions = [
            (agent_a_id, agent_b_id, &a_inv, &b_inv),
            (agent_b_id, agent_a_id, &b_inv, &a_inv),
        ];

        'outer: for (teacher_id, student_id, t_inv, s_inv) in teach_directions {
            for &k_id in &all_knowledges {
                if t_inv.contains_key(&k_id) && !s_inv.contains_key(&k_id) {
                    if let Some(payment_food) = tuition_foods.into_iter().find(|f| s_inv.get(f).copied().unwrap_or(0) >= 2) {
                        if let Some(t) = agent_store.get_human_mut(teacher_id) { t.add_item(payment_food, 2); }
                        if let Some(s) = agent_store.get_human_mut(student_id) {
                            let _ = s.remove_item(payment_food, 2);
                            s.add_item(k_id, 1);
                        }
                        let inst_k = ItemInstance::new(k_id, 1, serde_json::json!({"nature": "NonRivalKnowledge"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                        *next_instance_id += 1;
                        let inst_food = ItemInstance::new(payment_food, 2, serde_json::json!({"nature": "RivalPhysical"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                        *next_instance_id += 1;
                        let entry = LedgerEntry::new(*next_trx_id, run_id.clone(), current_tick, teacher_id, student_id, vec![inst_k], vec![inst_food], serde_json::json!({
                            "transaction_type": "knowledge_service_trade", "knowledge_item_id": k_id.0, "teacher_id": teacher_id.0, "student_id": student_id.0, "tuition_paid_item_id": payment_food.0, "tuition_paid_qty": 2
                        }));
                        *next_trx_id += 1;
                        let _ = ledger_store.record(entry);
                        knowledge_trade_occurred = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    // Case C: Bilateral Physical Goods Barter (if no service occurred)
    let mut barter_occurred = false;
    if !medical_service_occurred && !knowledge_trade_occurred {
        let a_informed = agent_has_market_access(agent_a_id, agent_store);
        let b_informed = agent_has_market_access(agent_b_id, agent_store);

        let mult_a = |id: ItemId| if a_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };
        let mult_b = |id: ItemId| if b_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };

        let a_offer = a_inv
            .iter()
            .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
            .map(|(id, qty)| {
                let u = evaluate_marginal_utility(*id, a_cal, *qty) * mult_a(*id);
                (*id, u)
            })
            .min_by(|(_, u1), (_, u2)| u1.partial_cmp(u2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id);

        let b_offer = b_inv
            .iter()
            .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
            .map(|(id, qty)| {
                let u = evaluate_marginal_utility(*id, b_cal, *qty) * mult_b(*id);
                (*id, u)
            })
            .min_by(|(_, u1), (_, u2)| u1.partial_cmp(u2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id);

        if let (Some(item_a), Some(item_b)) = (a_offer, b_offer)
            && item_a != item_b
        {
            let a_stock_a = a_inv.get(&item_a).copied().unwrap_or(0);
            let a_stock_b = a_inv.get(&item_b).copied().unwrap_or(0);
            let b_stock_a = b_inv.get(&item_a).copied().unwrap_or(0);
            let b_stock_b = b_inv.get(&item_b).copied().unwrap_or(0);

            // Carl Menger's Saleability (Absatzfähigkeit): durable/universal goods carry a liquidity premium
            let liquidity_premium = |id: ItemId| -> f64 {
                if id == ItemId::WAREHOUSE_RECEIPT {
                    1.75 // Highest saleability: fully backed warehouse certificate, zero weight
                } else if id == ItemId::CLAY_TABLET {
                    1.60 // High saleability promissory debt token / proto-paper currency
                } else if id == ItemId::SHELLS || id == ItemId::SALT {
                    1.40 // High saleability commodity currency premium
                } else if id == ItemId::GRAIN {
                    1.15 // Staple currency backup
                } else {
                    1.0
                }
            };

            let u_a_gives = evaluate_marginal_utility(item_a, a_cal, a_stock_a) * mult_a(item_a);
            let u_a_receives = evaluate_marginal_utility(item_b, a_cal, a_stock_b) * mult_a(item_b) * liquidity_premium(item_b);
            let u_b_gives = evaluate_marginal_utility(item_b, b_cal, b_stock_b) * mult_b(item_b);
            let u_b_receives = evaluate_marginal_utility(item_a, b_cal, b_stock_a) * mult_b(item_a) * liquidity_premium(item_a);

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

                let is_indirect = item_a == ItemId::SALT || item_a == ItemId::SHELLS || item_a == ItemId::CLAY_TABLET || item_a == ItemId::WAREHOUSE_RECEIPT
                    || item_b == ItemId::SALT || item_b == ItemId::SHELLS || item_b == ItemId::CLAY_TABLET || item_b == ItemId::WAREHOUSE_RECEIPT;

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
                        "is_indirect_exchange": is_indirect,
                        "information_asymmetry": a_informed != b_informed,
                    }),
                );
                *next_trx_id += 1;
                let _ = ledger_store.record(entry);
                barter_occurred = true;
            }
        }
    }

    // Case D: Depository Banking & Warehouse Receipts (Pottery Jar & Basket Custodians)
    let mut banking_or_credit = false;
    if !medical_service_occurred && !knowledge_trade_occurred {
        // D.1 Deposit surplus grain into storage container for warehouse receipt
        let has_storage = |inv: &std::collections::BTreeMap<ItemId, u32>| -> bool {
            inv.contains_key(&ItemId::POTTERY_JAR) || inv.contains_key(&ItemId::WOVEN_BASKET)
        };
        for (custodian_id, depositor_id, c_inv, d_inv) in [(agent_a_id, agent_b_id, &a_inv, &b_inv), (agent_b_id, agent_a_id, &b_inv, &a_inv)] {
            if has_storage(c_inv) && d_inv.get(&ItemId::GRAIN).copied().unwrap_or(0) >= 2 {
                if let Some(dep) = agent_store.get_human_mut(depositor_id) {
                    let _ = dep.remove_item(ItemId::GRAIN, 2);
                    dep.add_item(ItemId::WAREHOUSE_RECEIPT, 1);
                }
                if let Some(cust) = agent_store.get_human_mut(custodian_id) {
                    cust.add_item(ItemId::GRAIN, 2);
                }
                let inst = ItemInstance::new(ItemId::WAREHOUSE_RECEIPT, 1, serde_json::json!({"nature": "DepositoryReceipt", "grain_deposit": 2}))
                    .with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;
                let entry = LedgerEntry::single(*next_trx_id, run_id.clone(), current_tick, depositor_id, custodian_id, Some(inst), None, serde_json::json!({
                    "transaction_type": "granary_depository_banking", "custodian_id": custodian_id.0, "depositor_id": depositor_id.0, "grain_deposited": 2
                }));
                *next_trx_id += 1;
                let _ = ledger_store.record(entry);
                banking_or_credit = true;
                break;
            }
        }

        // D.2 Redemption of Warehouse Receipt for grain
        if !banking_or_credit {
            for (holder_id, cust_id, h_inv, c_inv) in [(agent_a_id, agent_b_id, &a_inv, &b_inv), (agent_b_id, agent_a_id, &b_inv, &a_inv)] {
                if h_inv.contains_key(&ItemId::WAREHOUSE_RECEIPT) && has_storage(c_inv) && c_inv.get(&ItemId::GRAIN).copied().unwrap_or(0) >= 2 {
                    if let Some(h) = agent_store.get_human_mut(holder_id) {
                        let _ = h.remove_item(ItemId::WAREHOUSE_RECEIPT, 1);
                        h.add_item(ItemId::GRAIN, 2);
                    }
                    if let Some(c) = agent_store.get_human_mut(cust_id) {
                        let _ = c.remove_item(ItemId::GRAIN, 2);
                    }
                    let entry = LedgerEntry::single(*next_trx_id, run_id.clone(), current_tick, holder_id, cust_id, None, None, serde_json::json!({
                        "transaction_type": "warehouse_receipt_redemption", "holder_id": holder_id.0, "custodian_id": cust_id.0, "grain_redeemed": 2
                    }));
                    *next_trx_id += 1;
                    let _ = ledger_store.record(entry);
                    banking_or_credit = true;
                    break;
                }
            }
        }

        // D.3 Emergency Food Credit Loan & Promissory Debt Token Minting
        let loan_foods = [ItemId::FLATBREAD, ItemId::GRAIN, ItemId::CURED_MEAT, ItemId::SMOKED_MEAT, ItemId::CURED_FISH];
        if !banking_or_credit {
            for (cred_id, deb_id, c_inv, d_cal) in [(agent_b_id, agent_a_id, &b_inv, a_cal), (agent_a_id, agent_b_id, &a_inv, b_cal)] {
                if d_cal < 3500.0 {
                    if let Some(&food_id) = loan_foods.iter().find(|&&f| c_inv.get(&f).copied().unwrap_or(0) >= 4) {
                        if let Some(cred) = agent_store.get_human_mut(cred_id) {
                            let _ = cred.remove_item(food_id, 2);
                            cred.add_item(ItemId::CLAY_TABLET, 1);
                        }
                        if let Some(deb) = agent_store.get_human_mut(deb_id) {
                            deb.add_item(food_id, 2);
                        }
                        let inst = ItemInstance::new(food_id, 2, serde_json::json!({"nature": "CreditLoan"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                        *next_instance_id += 1;
                        let entry = LedgerEntry::single(*next_trx_id, run_id.clone(), current_tick, cred_id, deb_id, Some(inst), None, serde_json::json!({
                            "transaction_type": "granary_credit_loan", "creditor_id": cred_id.0, "debtor_id": deb_id.0, "food_id": food_id.0, "principal": 2
                        }));
                        *next_trx_id += 1;
                        let _ = ledger_store.record(entry);
                        banking_or_credit = true;
                        break;
                    }
                }
            }
        }

        // D.4 Redemption of Promissory Debt Tablet
        if !banking_or_credit {
            for (holder_id, redeemer_id, h_inv, r_inv) in [(agent_a_id, agent_b_id, &a_inv, &b_inv), (agent_b_id, agent_a_id, &b_inv, &a_inv)] {
                if h_inv.contains_key(&ItemId::CLAY_TABLET) {
                    if let Some(&f) = loan_foods.iter().find(|&&f| r_inv.get(&f).copied().unwrap_or(0) >= 4) {
                        if let Some(h) = agent_store.get_human_mut(holder_id) {
                            let _ = h.remove_item(ItemId::CLAY_TABLET, 1);
                            h.add_item(f, 2);
                        }
                        if let Some(r) = agent_store.get_human_mut(redeemer_id) {
                            let _ = r.remove_item(f, 2);
                        }
                        let entry = LedgerEntry::single(*next_trx_id, run_id.clone(), current_tick, holder_id, redeemer_id, None, None, serde_json::json!({
                            "transaction_type": "promissory_tablet_redemption", "holder_id": holder_id.0, "redeemer_id": redeemer_id.0, "food_id": f.0
                        }));
                        *next_trx_id += 1;
                        let _ = ledger_store.record(entry);
                        banking_or_credit = true;
                        break;
                    }
                }
            }
        }
    }

    // Case E: Emergent Coasean Firm Coalition / Production Partnership & Wage Employment
    if !medical_service_occurred && !knowledge_trade_occurred && !banking_or_credit {
        let mut firm_occurred = false;
        // E.1 Milling Joint Venture (50:50 share of milled flour)
        for (cap_id, lab_id, cap_inv, lab_inv) in [(agent_a_id, agent_b_id, &a_inv, &b_inv), (agent_b_id, agent_a_id, &b_inv, &a_inv)] {
            if cap_inv.contains_key(&ItemId::SADDLE_QUERN) && lab_inv.get(&ItemId::GRAIN).copied().unwrap_or(0) >= 2 {
                if let Some(lab) = agent_store.get_human_mut(lab_id) {
                    let _ = lab.remove_item(ItemId::GRAIN, 2);
                    lab.add_item(ItemId::GRAIN_FLOUR, 1);
                }
                if let Some(cap) = agent_store.get_human_mut(cap_id) {
                    cap.add_item(ItemId::GRAIN_FLOUR, 1);
                }
                let inst_c = ItemInstance::new(ItemId::GRAIN_FLOUR, 1, serde_json::json!({"firm_role": "CapitalOwnerShare"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;
                let inst_l = ItemInstance::new(ItemId::GRAIN_FLOUR, 1, serde_json::json!({"firm_role": "LaborerShare"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                *next_instance_id += 1;
                let entry = LedgerEntry::new(*next_trx_id, run_id.clone(), current_tick, cap_id, lab_id, vec![inst_c], vec![inst_l], serde_json::json!({
                    "transaction_type": "firm_production_partnership", "enterprise": "Saddle Quern Grain Milling Joint Venture", "capitalist_id": cap_id.0, "laborer_id": lab_id.0
                }));
                *next_trx_id += 1;
                let _ = ledger_store.record(entry);
                firm_occurred = true;
                break;
            }
        }

        // E.2 Coasean Firm Wage Contract (Capitalist advances wage food, employs laborer with tools)
        if !firm_occurred {
            for (cap_id, lab_id, cap_inv, lab_cal) in [(agent_a_id, agent_b_id, &a_inv, b_cal), (agent_b_id, agent_a_id, &b_inv, a_cal)] {
                if lab_cal < 3500.0 && (cap_inv.contains_key(&ItemId::STONE_AXE) || cap_inv.contains_key(&ItemId::HUNTING_SPEAR)) {
                    let wage_food = [ItemId::FLATBREAD, ItemId::GRAIN_FLOUR, ItemId::CURED_FISH, ItemId::SMOKED_MEAT, ItemId::GRAIN]
                        .into_iter().find(|&f| cap_inv.get(&f).copied().unwrap_or(0) >= 2);
                    if let Some(wage_item) = wage_food {
                        let (produced_item, qty) = if cap_inv.contains_key(&ItemId::STONE_AXE) {
                            (ItemId::TIMBER, 2)
                        } else {
                            (ItemId::RAW_MEAT, 1)
                        };
                        if let Some(cap) = agent_store.get_human_mut(cap_id) {
                            let _ = cap.remove_item(wage_item, 1);
                            cap.add_item(produced_item, qty);
                        }
                        if let Some(lab) = agent_store.get_human_mut(lab_id) {
                            lab.add_item(wage_item, 1);
                        }
                        let inst_w = ItemInstance::new(wage_item, 1, serde_json::json!({"role": "WageAdvance"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                        *next_instance_id += 1;
                        let inst_p = ItemInstance::new(produced_item, qty, serde_json::json!({"role": "EnterpriseProduct"})).with_instance_id(ItemInstanceId::new(*next_instance_id));
                        *next_instance_id += 1;
                        let entry = LedgerEntry::new(*next_trx_id, run_id.clone(), current_tick, cap_id, lab_id, vec![inst_p], vec![inst_w], serde_json::json!({
                            "transaction_type": "firm_wage_employment", "capitalist_id": cap_id.0, "laborer_id": lab_id.0, "wage_item_id": wage_item.0, "product_id": produced_item.0
                        }));
                        *next_trx_id += 1;
                        let _ = ledger_store.record(entry);
                        break;
                    }
                }
            }
        }
    }
}

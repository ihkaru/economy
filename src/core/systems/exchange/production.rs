use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::EconomicActor;
use crate::core::domain::item::id::{ItemId, ItemInstanceId};
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::production::RecipeRegistry;
use crate::core::domain::time::{RunId, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::rng_port::RngPort;

/// Executes agent-centric autonomous crafting and multi-tier supply chain production
pub fn perform_autonomous_crafting(
    run_id: &RunId,
    current_tick: Tick,
    living_agent_ids: &[AgentId],
    agent_store: &mut dyn AgentStorePort,
    ledger_store: &mut dyn LedgerStorePort,
    rng: &mut dyn RngPort,
    next_trx_id: &mut u64,
    next_instance_id: &mut u64,
) {
    let registry = RecipeRegistry::canonical();

    for &agent_id in living_agent_ids {
        let (calorie_reserve, is_sick, inventory_snapshot) = {
            if let Some(agent) = agent_store.get_human(agent_id) {
                (agent.calorie_reserve, agent.is_sick(), agent.inventory.clone())
            } else {
                continue;
            }
        };

        // Starving agents lack the metabolic energy budget to invest in manufacturing labor
        if calorie_reserve < 2500.0 {
            continue;
        }

        for recipe in registry.all() {
            // 1. Knowledge prerequisite check (Non-Rival Blueprint)
            if let Some(k_id) = recipe.required_knowledge {
                if !inventory_snapshot.contains_key(&k_id) {
                    continue;
                }
            }

            // 2. Catalyst tool prerequisite check (Auxiliary Capital)
            if let Some(t_id) = recipe.required_tool {
                if !inventory_snapshot.contains_key(&t_id) {
                    continue;
                }
            }

            // 3. Leontief input-output material sufficiency check
            let mut has_all_inputs = true;
            for ing in recipe.inputs {
                let current_qty = inventory_snapshot.get(&ing.item_id).copied().unwrap_or(0);
                if current_qty < ing.quantity {
                    has_all_inputs = false;
                    break;
                }
            }
            if !has_all_inputs {
                continue;
            }

            // 4. Emergent marginal utility & need evaluation
            let primary_output_id = recipe.outputs.first().map(|o| o.item_id).unwrap_or(ItemId(0));
            let current_holding = inventory_snapshot.get(&primary_output_id).copied().unwrap_or(0);

            let should_craft = match primary_output_id {
                ItemId::WOVEN_BASKET => current_holding < 2,
                ItemId::STONE_AXE | ItemId::FISHING_NET | ItemId::RAFT | ItemId::HUNTING_SPEAR | ItemId::SADDLE_QUERN => current_holding == 0,
                ItemId::HERBAL_MEDICINE => is_sick || current_holding < 2,
                ItemId::CURED_FISH | ItemId::CURED_MEAT | ItemId::SMOKED_FISH | ItemId::SMOKED_MEAT => current_holding < 8,
                ItemId::DRIED_BERRIES => current_holding < 6,
                ItemId::POTTERY_JAR => current_holding < 2,
                ItemId::LEATHER_CLOTHING => current_holding < 2,
                ItemId::GRAIN_FLOUR => current_holding < 6,
                ItemId::FLATBREAD => current_holding < 8,
                ItemId::CHARCOAL => current_holding < 4,
                ItemId::CLAY_TABLET => current_holding < 4,
                _ => current_holding < 3,
            };

            if !should_craft {
                continue;
            }

            // 5. Execute production transformation
            let mut spent_instances = Vec::new();
            let mut produced_instances = Vec::new();

            if let Some(agent) = agent_store.get_human_mut(agent_id) {
                // Deduct inputs
                for ing in recipe.inputs {
                    let _ = agent.remove_item(ing.item_id, ing.quantity);
                    let instance = ItemInstance::new(
                        ing.item_id,
                        ing.quantity,
                        serde_json::json!({"nature": "ConsumedRawMaterial"}),
                    )
                    .with_instance_id(ItemInstanceId::new(*next_instance_id));
                    *next_instance_id += 1;
                    spent_instances.push(instance);
                }

                // Add outputs
                for out in recipe.outputs {
                    agent.add_item(out.item_id, out.quantity);
                    let instance = ItemInstance::new(
                        out.item_id,
                        out.quantity,
                        serde_json::json!({
                            "recipe_id": recipe.recipe_id,
                            "recipe_name": recipe.name,
                        }),
                    )
                    .with_instance_id(ItemInstanceId::new(*next_instance_id));
                    *next_instance_id += 1;
                    produced_instances.push(instance);
                }

                // Burn labor calories
                agent.calorie_reserve = (agent.calorie_reserve - recipe.labor_calorie_cost).max(500.0);

                // Tool wear-and-tear
                if let Some(t_id) = recipe.required_tool {
                    if recipe.tool_wear_probability > 0.0 && rng.check_probability(recipe.tool_wear_probability) {
                        let _ = agent.remove_item(t_id, 1);
                    }
                }
            }

            // 6. Record atomic transformation in Ultimate Ledger
            let entry = LedgerEntry::new(
                *next_trx_id,
                run_id.clone(),
                current_tick,
                agent_id,
                agent_id,
                spent_instances,
                produced_instances,
                serde_json::json!({
                    "transaction_type": recipe.category,
                    "product_name": recipe.name,
                    "recipe_id": recipe.recipe_id,
                    "labor_calories": recipe.labor_calorie_cost,
                }),
            );
            *next_trx_id += 1;
            let _ = ledger_store.record(entry);

            // Agent dedicates work time to one manufacturing project per tick
            break;
        }
    }
}

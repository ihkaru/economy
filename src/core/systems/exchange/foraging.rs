use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::EconomicActor;
use crate::core::domain::item::id::{ItemId, ItemInstanceId};
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::time::{RunId, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::rng_port::RngPort;

/// Executes agent-centric optimal foraging and autonomous crafting under biophysical constraints
pub fn perform_agent_centric_foraging(
    run_id: &RunId,
    current_tick: Tick,
    living_agent_ids: &[AgentId],
    agent_store: &mut dyn AgentStorePort,
    env_store: &mut dyn EnvironmentStorePort,
    ledger_store: &mut dyn LedgerStorePort,
    rng: &mut dyn RngPort,
    next_trx_id: &mut u64,
    next_instance_id: &mut u64,
) {
    for &agent_id in living_agent_ids {
        let (loc, calorie_reserve, is_sick, inventory_weight, capacity, timber_count, herb_count, clay_count, stone_count, has_axe) = {
            if let Some(agent) = agent_store.get_human(agent_id) {
                (
                    agent.location,
                    agent.calorie_reserve,
                    agent.is_sick(),
                    agent.inventory_weight_kg(),
                    agent.carrying_capacity_kg(),
                    agent.inventory.get(&ItemId::TIMBER).copied().unwrap_or(0),
                    agent.inventory.get(&ItemId::HERBAL_MEDICINE).copied().unwrap_or(0),
                    agent.inventory.get(&ItemId::CLAY).copied().unwrap_or(0),
                    agent.inventory.get(&ItemId::STONE).copied().unwrap_or(0),
                    agent.has_item(ItemId::STONE_AXE),
                )
            } else {
                continue;
            }
        };

        // If carrying capacity is fully saturated (less than 0.5 kg remaining), agent cannot forage raw goods
        let remaining_capacity_kg = capacity - inventory_weight;
        if remaining_capacity_kg < 0.2 {
            continue;
        }

        // 1. Evaluate candidate resource nodes using Charnov's Marginal Value Theorem
        let mut best_node_id = None;
        let mut best_score = 0.0_f64;
        let mut best_tool_multiplier = 1.0_f64;

        for node in env_store.nodes_mut() {
            if node.current_stock == 0 {
                continue;
            }

            // Spatial accessibility (within 5 cells radius, or across water)
            let distance = loc.euclidean_distance(&node.location);
            if distance > 5.0 {
                continue;
            }

            // Calculate capital tool efficiency
            let tool_multiplier = if let Some(agent) = agent_store.get_human(agent_id) {
                if node.item_id == ItemId::TIMBER && agent.has_item(ItemId::STONE_AXE) {
                    3.0
                } else if node.item_id == ItemId::FISH && agent.has_item(ItemId::FISHING_NET) {
                    3.0
                } else {
                    1.0
                }
            } else {
                1.0
            };

            // Stock abundance ratio (Charnov Marginal Value Theorem: patch departure when returns diminish)
            let abundance_ratio = (node.maturity * node.maturity).clamp(0.02, 1.0);

            // Physiological urgency weighting
            let urgency_weight = if is_sick && herb_count == 0 && node.item_id == ItemId::HERBAL_MEDICINE {
                15.0 // Desperate need for medicine to cure illness
            } else if calorie_reserve < 3500.0 && node.is_edible {
                10.0 // Hungry/starving: food has maximum marginal utility
            } else if timber_count < 3 && node.item_id == ItemId::TIMBER {
                6.0 // Firewood needed for thermoregulation against cold
            } else if node.is_edible && calorie_reserve < 6000.0 {
                4.0 // Well-fed maintenance buffer
            } else if node.item_id == ItemId::TIMBER && timber_count < 10 {
                3.0 // Raw material for tool/basket crafting
            } else if node.item_id == ItemId::SALT {
                2.5 // Food preservation medium
            } else if node.item_id == ItemId::HERBAL_MEDICINE && herb_count < 3 {
                2.0 // Small preventive medical stock
            } else if node.item_id == ItemId::STONE && (!has_axe || stone_count < 2) {
                3.2 // Raw material for Stone Axe crafting
            } else if node.item_id == ItemId::CLAY && clay_count < 6 {
                2.2 // Raw material for ceramic pottery crafting
            } else {
                0.2 // Hoarding disincentive for saturated goods
            };

            let score = urgency_weight * tool_multiplier * abundance_ratio;
            if score > best_score {
                best_score = score;
                best_node_id = Some(node.id);
                best_tool_multiplier = tool_multiplier;
            }
        }

        // 2. Execute harvest on chosen best node if score is viable
        if let Some(target_node_id) = best_node_id {
            if best_score > 0.5 {
                let target_node = env_store
                    .nodes_mut()
                    .iter_mut()
                    .find(|n| n.id == target_node_id);

                if let Some(node) = target_node {
                    let unit_weight = node.item_id.default_weight_kg().max(0.1);
                    let max_units_by_weight = (remaining_capacity_kg / unit_weight).floor() as u32;

                    if max_units_by_weight > 0 {
                        let harvest_attempt = rng.gen_range_u64(1, 3).min(max_units_by_weight as u64) as u32;
                        let actual_harvested = node.harvest(harvest_attempt.max(1), best_tool_multiplier);

                        if actual_harvested > 0 {
                            if let Some(agent) = agent_store.get_human_mut(agent_id) {
                                agent.add_item(node.item_id, actual_harvested);

                                // Hunting by-product: animal raw hide from terrestrial game hunting
                                if node.item_id == ItemId::RAW_MEAT && rng.check_probability(0.50) {
                                    agent.add_item(ItemId::RAW_HIDE, 1);
                                }

                                // Tool wear-and-tear degradation
                                if best_tool_multiplier > 1.0 {
                                    if node.item_id == ItemId::TIMBER && rng.check_probability(0.025) {
                                        let _ = agent.remove_item(ItemId::STONE_AXE, 1);
                                    } else if node.item_id == ItemId::FISH && rng.check_probability(0.02) {
                                        let _ = agent.remove_item(ItemId::FISHING_NET, 1);
                                    }
                                }
                            }

                            // Record transaction in Ultimate Ledger
                            let item_instance = ItemInstance::new(
                                node.item_id,
                                actual_harvested,
                                serde_json::json!({
                                    "source_node": node.name,
                                    "resource_id": node.id,
                                    "tool_multiplier": best_tool_multiplier,
                                    "node_maturity": node.maturity,
                                }),
                            )
                            .with_instance_id(ItemInstanceId::new(*next_instance_id));
                            *next_instance_id += 1;

                            let entry = LedgerEntry::single(
                                *next_trx_id,
                                run_id.clone(),
                                current_tick,
                                AgentId(0), // Nature
                                agent_id,
                                Some(item_instance),
                                None,
                                serde_json::json!({
                                    "transaction_type": "natural_resource_harvest",
                                    "node_name": node.name,
                                    "tool_multiplier": best_tool_multiplier,
                                    "node_maturity": node.maturity,
                                    "remaining_node_stock": node.current_stock
                                }),
                            );
                            *next_trx_id += 1;
                            let _ = ledger_store.record(entry);
                        }
                    }
                }
            }
        }
    }
}


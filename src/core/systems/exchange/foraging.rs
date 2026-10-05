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

use crate::core::domain::spatial::coordinate::GeoCoordinate;

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
    let settlement_loc = GeoCoordinate::new(15, 25);

    for &agent_id in living_agent_ids {
        let (loc, calorie_reserve, is_sick, inventory_weight, capacity, timber_count, herb_count, clay_count, stone_count, salt_count, shell_count, meat_fish_count, food_count, has_axe, has_spear, has_raft) = {
            if let Some(agent) = agent_store.get_human(agent_id) {
                let meat_fish = agent.inventory.get(&ItemId::RAW_MEAT).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::FISH).copied().unwrap_or(0);
                let preserved_food = agent.inventory.get(&ItemId::CURED_FISH).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::CURED_MEAT).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::SMOKED_FISH).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::SMOKED_MEAT).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::DRIED_BERRIES).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::FLATBREAD).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::GRAIN_FLOUR).copied().unwrap_or(0);
                let food = agent.inventory.get(&ItemId::BERRIES).copied().unwrap_or(0)
                    + agent.inventory.get(&ItemId::GRAIN).copied().unwrap_or(0)
                    + meat_fish
                    + preserved_food;
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
                    agent.inventory.get(&ItemId::SALT).copied().unwrap_or(0),
                    agent.inventory.get(&ItemId::SHELLS).copied().unwrap_or(0),
                    meat_fish,
                    food,
                    agent.has_item(ItemId::STONE_AXE),
                    agent.has_item(ItemId::HUNTING_SPEAR),
                    agent.has_item(ItemId::RAFT),
                )
            } else {
                continue;
            }
        };

        // If carrying capacity is fully saturated (less than 0.5 kg remaining):
        // Well-fed agents step back toward central settlement to trade or deposit goods
        let remaining_capacity_kg = capacity - inventory_weight;
        if remaining_capacity_kg < 0.5 {
            if calorie_reserve >= 4500.0 && food_count >= 2 {
                if loc != settlement_loc {
                    if let Some(agent) = agent_store.get_human_mut(agent_id) {
                        agent.location = agent.location.step_towards(&settlement_loc);
                    }
                }
                continue;
            }

            // Food-insecure or hungry agents discard heavy raw mineral deadweight to make room for subsistence
            if let Some(agent) = agent_store.get_human_mut(agent_id) {
                if agent.inventory.get(&ItemId::STONE).copied().unwrap_or(0) > 4 {
                    let _ = agent.remove_item(ItemId::STONE, 2);
                } else if agent.inventory.get(&ItemId::ANIMAL_BONE).copied().unwrap_or(0) > 4 {
                    let _ = agent.remove_item(ItemId::ANIMAL_BONE, 2);
                } else if agent.inventory.get(&ItemId::CLAY).copied().unwrap_or(0) > 4 {
                    let _ = agent.remove_item(ItemId::CLAY, 2);
                }
            }
        }

        // 1. Evaluate candidate resource nodes using Charnov's Marginal Value Theorem
        let mut best_node_id = None;
        let mut best_score = 0.0_f64;
        let mut best_tool_multiplier = 1.0_f64;
        let mut best_node_loc = loc;

        for node in env_store.nodes_mut() {
            if node.current_stock == 0 {
                continue;
            }

            // Spatial accessibility (within 8 cells radius on mainland, or offshore if raft available)
            let distance = loc.euclidean_distance(&node.location);
            if distance > 8.0 && !has_raft {
                continue;
            }

            // Calculate capital tool & learning-by-doing efficiency (Arrow 1962 / Adam Smith)
            let tool_multiplier = if let Some(agent) = agent_store.get_human(agent_id) {
                let base_mult = if node.item_id == ItemId::TIMBER {
                    if agent.has_item(ItemId::STONE_AXE) { 3.0 } else { 0.25 }
                } else if node.item_id == ItemId::FISH {
                    if agent.has_item(ItemId::FISHING_NET) { 3.0 } else { 0.25 }
                } else if node.item_id == ItemId::RAW_MEAT {
                    if agent.has_item(ItemId::HUNTING_SPEAR) { 3.0 } else { 0.20 }
                } else {
                    1.0
                };
                let spec_key = node.item_id.0.to_string();
                let exp = agent.attributes.get("specialization")
                    .and_then(|s| s.get(&spec_key))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                (base_mult + (exp as f64 * 0.05)).min(5.0)
            } else {
                1.0
            };

            // Stock abundance ratio (Charnov Marginal Value Theorem: patch departure when returns diminish)
            let abundance_ratio = (node.maturity * node.maturity).clamp(0.02, 1.0);

            // Physiological urgency weighting & Gossen's diminishing marginal utility
            let urgency_weight = if is_sick && herb_count == 0 && node.item_id == ItemId::HERBAL_MEDICINE {
                15.0 // Desperate need for medicine to cure illness
            } else if (calorie_reserve < 4500.0 || food_count < 2) && node.is_edible {
                9.0 // Food security buffer: agents ensure survival before mineral/currency expeditions
            } else if timber_count < 3 && node.item_id == ItemId::TIMBER {
                6.0 // Firewood needed for thermoregulation against cold
            } else if meat_fish_count > 0 && salt_count < 2 && node.item_id == ItemId::SALT {
                5.5 // Urgent preservation: Salt needed to cure perishable meat/fish before spoilage
            } else if node.item_id == ItemId::STONE && stone_count < 4 && (!has_axe || !has_spear || stone_count < 2) {
                4.5 // Raw material for Stone Axe and Hunting Spear crafting
            } else if node.item_id == ItemId::CLAY && clay_count < 4 {
                3.8 // Raw material for ceramic pottery jars and clay debt tablets
            } else if node.item_id == ItemId::SHELLS && shell_count < 6 {
                3.5 // Ancient maritime commodity currency (Carl Menger saleability)
            } else if node.item_id == ItemId::SALT && salt_count < 4 {
                3.2 // Food preservation medium and high-liquidity store of value
            } else if node.item_id == ItemId::TIMBER && timber_count < 8 {
                3.0 // Raw material for tool/basket/raft crafting
            } else if node.is_edible && food_count < 8 {
                3.0 // Well-fed maintenance buffer and market exchange surplus
            } else if node.item_id == ItemId::HERBAL_MEDICINE && herb_count < 2 {
                2.0 // Small preventive medical stock
            } else {
                0.3 // Hoarding disincentive for saturated goods (diminishing marginal utility)
            };

            let score = urgency_weight * tool_multiplier * abundance_ratio;
            if score > best_score {
                best_score = score;
                best_node_id = Some(node.id);
                best_tool_multiplier = tool_multiplier;
                best_node_loc = node.location;
            }
        }

        // 2. Execute harvest on chosen best node if score is viable
        if let Some(target_node_id) = best_node_id {
            if best_score > 0.25 {
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
                                agent.location = agent.location.step_towards(&best_node_loc);
                                agent.add_item(node.item_id, actual_harvested);

                                // Learning-by-Doing skill accumulation (Arrow 1962)
                                if let Some(obj) = agent.attributes.as_object_mut() {
                                    let spec = obj.entry("specialization".to_string()).or_insert_with(|| serde_json::json!({}));
                                    if let Some(spec_obj) = spec.as_object_mut() {
                                        let key = node.item_id.0.to_string();
                                        let count = spec_obj.get(&key).and_then(|v| v.as_u64()).unwrap_or(0);
                                        if count < 20 {
                                            spec_obj.insert(key, serde_json::json!(count + 1));
                                        }
                                    }
                                }

                                // Hunting by-products: animal raw hide & carcass bones from terrestrial game hunting (requires spear)
                                if node.item_id == ItemId::RAW_MEAT && agent.has_item(ItemId::HUNTING_SPEAR) {
                                    if rng.check_probability(0.50) {
                                        agent.add_item(ItemId::RAW_HIDE, 1);
                                    }
                                    if rng.check_probability(0.60) {
                                        agent.add_item(ItemId::ANIMAL_BONE, 1);
                                    }
                                }

                                // Tool wear-and-tear degradation
                                if best_tool_multiplier > 1.0 {
                                    if node.item_id == ItemId::TIMBER && rng.check_probability(0.025) {
                                        let _ = agent.remove_item(ItemId::STONE_AXE, 1);
                                    } else if node.item_id == ItemId::FISH && rng.check_probability(0.02) {
                                        let _ = agent.remove_item(ItemId::FISHING_NET, 1);
                                    } else if node.item_id == ItemId::RAW_MEAT && rng.check_probability(0.025) {
                                        let _ = agent.remove_item(ItemId::HUNTING_SPEAR, 1);
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


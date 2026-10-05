use crate::core::domain::agent::traits::{EconomicActor, HasLifecycle};
use crate::core::domain::item::id::ItemId;
use crate::core::domain::time::Tick;
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::rng_port::RngPort;

pub struct MetabolismSystem {
    base_daily_calories: f64,
}

impl MetabolismSystem {
    pub fn new(base_daily_calories: f64) -> Self {
        Self {
            base_daily_calories,
        }
    }

    /// Step metabolism, foraging for survival, calorie consumption, and starvation hazards
    pub fn step(
        &mut self,
        current_tick: Tick,
        agent_store: &mut dyn AgentStorePort,
        env_store: &mut dyn EnvironmentStorePort,
        rng: &mut dyn RngPort,
    ) {
        let living_ids = agent_store.living_human_ids();
        let climate = env_store.climate().clone();

        for id in living_ids {
            let (loc, calorie_balance, mut days_starving) = {
                if let Some(agent) = agent_store.get_human(id).filter(|a| a.is_alive()) {
                    (agent.location, agent.calorie_reserve, agent.days_starving)
                } else {
                    continue;
                }
            };

            // Calculate realistic spatial micro-climate with Environmental Lapse Rate (-6.5°C/1000m)
            let elevation = env_store
                .world_map()
                .and_then(|m| m.get_cell(loc))
                .map(|c| c.elevation_meters as f64)
                .unwrap_or_else(|| {
                    if loc.x >= 40 && loc.y == 25 {
                        950.0 // Volcanic Peak Island
                    } else if loc.y < 10 {
                        1800.0 // Northern Mountain Ridge
                    } else if loc.y > 35 {
                        200.0 // Southern Dense Forest
                    } else {
                        100.0 // Central Plains & River Valley
                    }
                });
            let local_temp = climate.local_temperature(elevation, loc.y, 50);

            // Cold weather penalty adjusted for local lapse rate
            let raw_cold_penalty = if local_temp < 10.0 {
                (10.0 - local_temp) * 40.0 // Extra kcal burned for homeothermy in the cold
            } else {
                0.0
            };

            // Thermoregulation: holding Timber (firewood) or wearing Leather Clothing protects against cold
            let (has_firewood, has_clothing) = agent_store
                .get_human(id)
                .map(|a| (a.has_item(ItemId::TIMBER), a.has_item(ItemId::LEATHER_CLOTHING)))
                .unwrap_or((false, false));
            let cold_penalty = if has_clothing {
                0.0 // Full thermal insulation against cold weather
            } else if has_firewood {
                raw_cold_penalty * 0.5
            } else {
                raw_cold_penalty
            };

            // Heat & Water Hydration: In high temperatures, agents need proximity to fresh water sources (River/ShallowWater)
            let is_near_water = if let Some(map) = env_store.world_map() {
                map.get_cell(loc).map(|c| c.terrain.is_water()).unwrap_or(false)
                    || loc.y.abs_diff(25) <= 5 // Proximity to central river valley
            } else {
                loc.y.abs_diff(25) <= 5
            };
            let heat_penalty = if local_temp > 30.0 && !is_near_water {
                (local_temp - 30.0) * 35.0 // Thermoregulatory dehydration stress
            } else {
                0.0
            };

            // 1. Food Consumption from Personal Inventory
            let mut calories_gained = 0.0;
            if let Some(agent) = agent_store.get_human_mut(id) {
                let edible_cal = [
                    (ItemId::FLATBREAD, 1200.0),    // Baked Flatbread (highest energy density & digestibility)
                    (ItemId::GRAIN_FLOUR, 500.0),   // Milled Flour
                    (ItemId::GRAIN, 400.0),         // Raw Grain (lower bio-availability uncooked)
                    (ItemId::FISH, 500.0),          // Fresh Fish (eat perishable first)
                    (ItemId::RAW_MEAT, 650.0),      // Fresh Terrestrial Game Meat
                    (ItemId::BERRIES, 300.0),       // Fresh Berries
                    (ItemId::SMOKED_FISH, 600.0),   // Wood-Smoked Preserved Fish
                    (ItemId::SMOKED_MEAT, 700.0),   // Wood-Smoked Preserved Meat
                    (ItemId::CURED_FISH, 650.0),    // Salt-Cured Preserved Fish
                    (ItemId::CURED_MEAT, 700.0),    // Salt-Cured Preserved Meat
                    (ItemId::DRIED_BERRIES, 400.0), // Sun-Dried Desiccated Berries
                ];

                for (item_id, cal_per_unit) in edible_cal {
                    if agent.calorie_reserve + calories_gained >= 6000.0 {
                        break;
                    }
                    if let Some(qty) = agent.inventory.get_mut(&item_id)
                        && *qty > 0
                    {
                        let units_needed = ((6000.0 - (agent.calorie_reserve + calories_gained)) / cal_per_unit).ceil() as u32;
                        let units_to_eat = (*qty).min(units_needed.max(1));
                        *qty -= units_to_eat;
                        calories_gained += units_to_eat as f64 * cal_per_unit;
                    }
                }
                agent.inventory.retain(|_, v| *v > 0);

                // Perishable food spoilage decay
                let has_salt = agent.has_item(ItemId::SALT);
                // Fresh fish & raw meat rot quickly without salt preservation (10% daily decay chance per unit)
                if !has_salt && agent.has_item(ItemId::FISH) && rng.check_probability(0.10) {
                    let _ = agent.remove_item(ItemId::FISH, 1);
                }
                if !has_salt && agent.has_item(ItemId::RAW_MEAT) && rng.check_probability(0.10) {
                    let _ = agent.remove_item(ItemId::RAW_MEAT, 1);
                }
                // Fresh berries rot if kept unconsumed (5% daily decay chance per unit)
                if agent.has_item(ItemId::BERRIES) && rng.check_probability(0.05) {
                    let _ = agent.remove_item(ItemId::BERRIES, 1);
                }
                // Stored grain damp & pest decay: Grain without sealed pottery jar rots slowly (0.1% daily decay chance)
                let has_pottery = agent.has_item(ItemId::POTTERY_JAR);
                if !has_pottery && agent.has_item(ItemId::GRAIN) && rng.check_probability(0.001) {
                    let _ = agent.remove_item(ItemId::GRAIN, 1);
                }

                // Material Entropy Spectrum: Passive Organic Aging & Weathering Decay
                // (Inorganic goods like Stone Axe, Shells, and Salt do NOT suffer passive decay and last decades/centuries)
                if agent.has_item(ItemId::WOVEN_BASKET) && rng.check_probability(0.002) {
                    let _ = agent.remove_item(ItemId::WOVEN_BASKET, 1);
                }
                if agent.has_item(ItemId::FISHING_NET) && rng.check_probability(0.002) {
                    let _ = agent.remove_item(ItemId::FISHING_NET, 1);
                }
                if agent.has_item(ItemId::RAFT) && rng.check_probability(0.001) {
                    let _ = agent.remove_item(ItemId::RAFT, 1);
                }
                if agent.has_item(ItemId::TIMBER) && rng.check_probability(0.002) {
                    let _ = agent.remove_item(ItemId::TIMBER, 1);
                }
                if agent.has_item(ItemId::HERBAL_MEDICINE) && rng.check_probability(0.003) {
                    let _ = agent.remove_item(ItemId::HERBAL_MEDICINE, 1);
                }
                if agent.has_item(ItemId::POTTERY_JAR) && rng.check_probability(0.0005) {
                    let _ = agent.remove_item(ItemId::POTTERY_JAR, 1);
                }
                if agent.has_item(ItemId::LEATHER_CLOTHING) && rng.check_probability(0.0005) {
                    let _ = agent.remove_item(ItemId::LEATHER_CLOTHING, 1);
                }
                if agent.has_item(ItemId::HUNTING_SPEAR) && rng.check_probability(0.001) {
                    let _ = agent.remove_item(ItemId::HUNTING_SPEAR, 1);
                }

                // Storage Spoilage Entropy: Perishables without container or salt suffer biological decay
                let has_container = agent.has_item(ItemId::POTTERY_JAR) || agent.has_item(ItemId::WOVEN_BASKET) || agent.has_item(ItemId::SALT);
                if !has_container {
                    for perishable in [ItemId::FISH, ItemId::BERRIES, ItemId::RAW_MEAT] {
                        if agent.has_item(perishable) && rng.check_probability(0.01) {
                            let _ = agent.remove_item(perishable, 1);
                        }
                    }
                }
            }

            // Liebig's Law of the Minimum: Electrolyte preservation via Salt
            // Holding Salt improves digestion assimilation efficiency (+10% caloric extraction)
            let has_salt = agent_store
                .get_human(id)
                .map(|a| a.has_item(ItemId::SALT))
                .unwrap_or(false);
            if has_salt {
                calories_gained *= 1.10;
            }

            // Disease / Infection status check
            let mut is_sick = false;
            let mut spouse_id_opt = None;
            if let Some(agent) = agent_store.get_human(id) {
                is_sick = agent.attributes.get("is_sick").and_then(|v| v.as_bool()).unwrap_or(false);
                spouse_id_opt = agent.spouse_id;
            }

            // Pathogen infection hazard from severe cold exposure or malnutrition
            if !is_sick {
                let infection_risk = if raw_cold_penalty > 0.0 && !has_clothing && !has_firewood {
                    0.02 // Chills & respiratory fever
                } else if days_starving > 0 {
                    0.03 // Opportunistic infection under starvation
                } else {
                    0.0
                };

                if infection_risk > 0.0 && rng.check_probability(infection_risk) {
                    is_sick = true;
                }
            }

            // Treatment & Healing
            if is_sick {
                if let Some(agent) = agent_store.get_human_mut(id) {
                    // Self-medication with Herbal Medicine
                    if agent.has_item(ItemId::HERBAL_MEDICINE) {
                        let _ = agent.remove_item(ItemId::HERBAL_MEDICINE, 1);
                        is_sick = false;
                    }
                }

                // Family Caregiving & Traditional Healing
                if is_sick {
                    if let Some(spouse_id) = spouse_id_opt {
                        if let Some(spouse) = agent_store.get_human(spouse_id) {
                            if (spouse.has_item(ItemId::SERVICE_MEDICAL)
                                || spouse.has_item(ItemId::KNOWLEDGE_HERBAL_MEDICINE))
                                && rng.check_probability(0.50)
                            {
                                is_sick = false;
                            }
                        }
                    }
                }

                // Natural immune recovery if well nourished
                if is_sick && (calorie_balance + calories_gained) > 5000.0 && rng.check_probability(0.15) {
                    is_sick = false;
                }
            }

            // 2. Parental Care: Young dependent children receive food from living parents
            let mut age_years = 20.0;
            if let Some(agent) = agent_store.get_human(id) {
                age_years = agent.age_ticks as f64 / 365.0;
                if age_years < 15.0 && (calorie_balance + calories_gained) < 3000.0 {
                    let parent_ids = [agent.mother_id, agent.father_id];
                    for p_id in parent_ids.into_iter().flatten() {
                        if let Some(parent) = agent_store.get_human_mut(p_id)
                            && parent.is_alive()
                            && parent.calorie_reserve > 1800.0
                        {
                            let share = 800.0_f64.min(parent.calorie_reserve - 1200.0);
                            if share > 0.0 {
                                parent.calorie_reserve -= share;
                                calories_gained += share;
                                break;
                            }
                        }
                    }
                }
            }

            // 3. Emergency Foraging from Nearby Resource Nodes if Still Hungry
            if calorie_balance + calories_gained < 6000.0 {
                for node in env_store.nodes_mut() {
                    if node.is_edible
                        && node.current_stock > 0
                        && loc.euclidean_distance(&node.location) <= 5.0
                    {
                        let needed = ((6000.0 - (calorie_balance + calories_gained)) / node.calories_per_unit).ceil() as u32;
                        let harvest_qty = needed.min(node.current_stock).min(4);
                        let harvested = node.harvest(harvest_qty, 1.0);
                        calories_gained += harvested as f64 * node.calories_per_unit;
                        if calorie_balance + calories_gained >= 6000.0 {
                            break;
                        }
                    }
                }
            }

            // 4. Caloric Expenditure Calculation (scaled for children) + local cold/heat/fever penalty
            let child_factor = if age_years < 5.0 {
                0.4
            } else if age_years < 12.0 {
                0.7
            } else {
                1.0
            };
            let fever_penalty = if is_sick { 300.0 } else { 0.0 };
            let encumbrance_penalty = if let Some(agent) = agent_store.get_human(id) {
                let w = agent.inventory_weight_kg();
                if w > 20.0 {
                    ((w - 20.0) / 10.0) * 50.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            let total_expenditure = (self.base_daily_calories * child_factor)
                + cold_penalty
                + heat_penalty
                + fever_penalty
                + encumbrance_penalty;

            // 5. Update agent state
            if let Some(agent) = agent_store.get_human_mut(id) {
                agent.calorie_reserve += calories_gained;

                if let Some(obj) = agent.attributes.as_object_mut() {
                    obj.insert("is_sick".to_string(), serde_json::json!(is_sick));
                }

                if agent.calorie_reserve >= total_expenditure {
                    agent.calorie_reserve -= total_expenditure;
                    agent.days_starving = 0; // Nourished
                } else {
                    agent.calorie_reserve = 0.0;
                    agent.days_starving += 1;
                    days_starving = agent.days_starving;
                }

                // 6. Starvation and illness mortality hazard
                if days_starving > 3 || (is_sick && days_starving > 1) {
                    let sick_multiplier = if is_sick { 1.5 } else { 1.0 };
                    let hazard = ((0.08 * (days_starving.saturating_sub(2)) as f64) * sick_multiplier).min(0.85);
                    if rng.check_probability(hazard) {
                        let cause = if is_sick {
                            format!("Illness complication and starvation after {} days without food", days_starving)
                        } else {
                            format!("Starvation after {} consecutive days without food", days_starving)
                        };
                        agent.mark_deceased(current_tick, cause);
                    }
                }
            }
        }
    }
}

impl Default for MetabolismSystem {
    fn default() -> Self {
        Self::new(2000.0)
    }
}

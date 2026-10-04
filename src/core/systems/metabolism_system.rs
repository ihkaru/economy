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
        let all_ids = agent_store.all_human_ids();
        let climate = env_store.climate().clone();

        for id in all_ids {
            let mut agent_location = None;
            let mut is_alive = false;
            let mut calorie_balance = 0.0;
            let mut days_starving = 0;

            if let Some(agent) = agent_store.get_human(id).filter(|a| a.is_alive()) {
                is_alive = true;
                agent_location = Some(agent.location);
                calorie_balance = agent.calorie_reserve;
                days_starving = agent.days_starving;
            }

            if !is_alive {
                continue;
            }

            let loc = agent_location.unwrap();

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

            // Firewood thermoregulation: holding Timber provides heating and halves cold penalty
            let has_firewood = agent_store
                .get_human(id)
                .map(|a| a.has_item(ItemId::TIMBER))
                .unwrap_or(false);
            let cold_penalty = if has_firewood { raw_cold_penalty * 0.5 } else { raw_cold_penalty };

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
                    (ItemId::GRAIN, 800.0),   // Grain
                    (ItemId::FISH, 500.0),    // Fish
                    (ItemId::BERRIES, 300.0), // Berries
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

            // 4. Caloric Expenditure Calculation (scaled for children) + local cold penalty
            let child_factor = if age_years < 5.0 {
                0.4
            } else if age_years < 12.0 {
                0.7
            } else {
                1.0
            };
            let total_expenditure = (self.base_daily_calories * child_factor) + cold_penalty + heat_penalty;

            // 5. Update agent state
            if let Some(agent) = agent_store.get_human_mut(id) {
                agent.calorie_reserve += calories_gained;

                if agent.calorie_reserve >= total_expenditure {
                    agent.calorie_reserve -= total_expenditure;
                    agent.days_starving = 0; // Nourished
                } else {
                    agent.calorie_reserve = 0.0;
                    agent.days_starving += 1;
                    days_starving = agent.days_starving;
                }

                // 6. Starvation mortality hazard
                if days_starving > 3 {
                    let hazard = (0.08 * (days_starving - 3) as f64).min(0.85);
                    if rng.check_probability(hazard) {
                        agent.mark_deceased(
                            current_tick,
                            format!("Starvation after {} consecutive days without food", days_starving),
                        );
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

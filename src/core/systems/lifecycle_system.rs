use crate::core::domain::agent::human::{Human, Sex};
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::{EconomicActor, HasLifecycle, SocialActor};
use crate::core::domain::time::{Tick, TickDuration};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::rng_port::RngPort;

pub struct LifecycleSystem {
    next_agent_id: u64,
}

impl LifecycleSystem {
    pub fn new(initial_next_id: u64) -> Self {
        Self {
            next_agent_id: initial_next_id,
        }
    }

    /// Step aging, marriage, reproduction, and mortality for all agents
    pub fn step(
        &mut self,
        current_tick: Tick,
        tick_duration: TickDuration,
        agent_store: &mut dyn AgentStorePort,
        rng: &mut dyn RngPort,
    ) {
        let living_ids = agent_store.living_human_ids();
        let fractional_years = tick_duration.fractional_years();

        // 1. Advance age for all living agents
        for id in &living_ids {
            if let Some(agent) = agent_store.get_human_mut(*id) {
                if agent.is_alive() {
                    agent.step_age(1);
                }
            }
        }

        // 2. Mortality Check (Gompertz-Makeham hazard model)
        for id in &living_ids {
            if let Some(agent) = agent_store.get_human_mut(*id) {
                if !agent.is_alive() {
                    continue;
                }

                let age_in_years = (agent.age_ticks as f64) * fractional_years;
                
                // Annual mortality probability using Gompertz-Makeham curve
                // Baseline hazard + pre-industrial infant & early child vulnerability (bathtub curve) + exponential senescence
                let early_childhood_hazard = if age_in_years < 1.0 {
                    0.08 // Pre-industrial infant vulnerability (~8% annual hazard)
                } else if age_in_years < 5.0 {
                    0.02 // Early childhood vulnerability (~2% annual hazard)
                } else {
                    0.0
                };
                let annual_mortality = (0.001 + early_childhood_hazard + 0.00008 * (1.095_f64).powf(age_in_years)).min(0.999);
                
                // Convert annual probability to per-tick probability: 1 - (1 - P_annual)^fractional_years
                let tick_mortality = 1.0 - (1.0 - annual_mortality).powf(fractional_years);

                if rng.check_probability(tick_mortality) {
                    agent.mark_deceased(current_tick, format!("Natural causes at age {:.1} years", age_in_years));
                }
            }
        }

        // 3. Estate Settlement & Widow Remarriage Clearance
        // Settle inventory inheritance and free surviving spouses for remarriage (only for agents deceased on current_tick)
        for id in &living_ids {
            let (is_deceased, spouse_opt, children, inventory_items) = match agent_store.get_human(*id) {
                Some(agent) if !agent.is_alive() && agent.death_tick() == Some(current_tick) => (
                    true,
                    agent.spouse_id,
                    agent.children_ids.clone(),
                    agent.inventory.clone(),
                ),
                _ => (false, None, Vec::new(), std::collections::BTreeMap::new()),
            };

            if is_deceased {
                // Find living heir: 1) living spouse, 2) living children
                let mut heir_id = None;
                if let Some(spouse) = spouse_opt {
                    if agent_store.get_human(spouse).map(|s| s.is_alive()).unwrap_or(false) {
                        heir_id = Some(spouse);
                    }
                }
                if heir_id.is_none() {
                    for child in &children {
                        if agent_store.get_human(*child).map(|c| c.is_alive()).unwrap_or(false) {
                            heir_id = Some(*child);
                            break;
                        }
                    }
                }
                // 3) Tribal customary inheritance: surviving adult kin in village
                if heir_id.is_none() {
                    heir_id = living_ids.iter().find(|&&lid| {
                        lid != *id && agent_store.get_human(lid).map(|a| a.is_alive() && a.age_ticks >= 15 * 365).unwrap_or(false)
                    }).copied();
                }

                // Transfer physical capital / inventory to heir (Zero Ex-Nihilo Conservation)
                if let Some(heir) = heir_id {
                    if !inventory_items.is_empty() {
                        if let Some(heir_agent) = agent_store.get_human_mut(heir) {
                            for (item_id, qty) in inventory_items {
                                *heir_agent.inventory.entry(item_id).or_insert(0) += qty;
                            }
                        }
                    }
                }

                // Clear inventory and spouse link on deceased agent
                if let Some(agent) = agent_store.get_human_mut(*id) {
                    agent.inventory.clear();
                    agent.spouse_id = None;
                }

                // If spouse is still alive, clear their spouse link so they can remarry
                if let Some(spouse) = spouse_opt {
                    if let Some(surviving_spouse) = agent_store.get_human_mut(spouse) {
                        if surviving_spouse.spouse_id == Some(*id) {
                            surviving_spouse.spouse_id = None;
                        }
                    }
                }
            }
        }

        // 4. Marriage Matching
        // Collect eligible living unmarried males and females of legal age (>= 18 years)
        let mut eligible_males = Vec::new();
        let mut eligible_females = Vec::new();

        for id in &living_ids {
            if let Some(agent) = agent_store.get_human(*id) {
                let age_years = (agent.age_ticks as f64) * fractional_years;
                if agent.is_alive() && agent.spouse_id.is_none() && (18.0..=65.0).contains(&age_years) {
                    match agent.sex {
                        Sex::Male => eligible_males.push(*id),
                        Sex::Female => eligible_females.push(*id),
                    }
                }
            }
        }

        // Deterministic pairing
        let pair_count = eligible_males.len().min(eligible_females.len());
        // Match probability calibrated per tick from annual marriage rate (35% annually)
        let annual_marriage_rate = 0.35_f64;
        let marriage_prob_per_tick = 1.0_f64 - (1.0_f64 - annual_marriage_rate).powf(fractional_years);

        for i in 0..pair_count {
            if rng.check_probability(marriage_prob_per_tick) {
                let male_id = eligible_males[i];
                let female_id = eligible_females[i];

                if let Some(male) = agent_store.get_human_mut(male_id) {
                    male.set_spouse(Some(female_id));
                }
                if let Some(female) = agent_store.get_human_mut(female_id) {
                    female.set_spouse(Some(male_id));
                }
            }
        }

        // 5. Childbirth (Reproduction)
        // Check married couples where female is between 18 and 45 years and has sufficient nutritional energy reserve
        let mut births_to_create = Vec::new();

        for id in &living_ids {
            if let Some(female) = agent_store.get_human(*id) {
                let age_years = (female.age_ticks as f64) * fractional_years;
                if female.is_alive()
                    && female.sex == Sex::Female
                    && (18.0..=45.0).contains(&age_years)
                    && female.days_starving == 0
                    && female.calorie_reserve >= 1800.0 // Malnutrition amenorrhea prevention
                    && let Some(spouse_id) = female.spouse_id
                {
                    // Ensure spouse is also alive
                    let spouse_alive = agent_store.get_human(spouse_id).map(|s| s.is_alive()).unwrap_or(false);
                    if !spouse_alive {
                        continue;
                    }

                    let annual_birth_rate: f64 = 0.35;
                    let tick_birth_prob = 1.0_f64 - (1.0_f64 - annual_birth_rate).powf(fractional_years);

                    if rng.check_probability(tick_birth_prob) {
                        // Natural biological sex ratio: ~105 males per 100 females (approx 51.2% male)
                        let newborn_sex = if rng.check_probability(105.0 / 205.0) {
                            Sex::Male
                        } else {
                            Sex::Female
                        };
                        births_to_create.push((newborn_sex, spouse_id, female.id, female.location));
                    }
                }
            }
        }

        // Spawn new human agents
        for (sex, father_id, mother_id, mother_loc) in births_to_create {
            let child_id = AgentId::new(self.next_agent_id);
            self.next_agent_id += 1;

            // Zero Ex-Nihilo energy conservation: mother transfers maternal caloric investment
            let mut mother_gen = 1;
            let maternal_investment = if let Some(mother) = agent_store.get_human_mut(mother_id) {
                mother_gen = mother.attributes.get("generation").and_then(|g| g.as_u64()).unwrap_or(1);
                let transfer = 800.0_f64.min(mother.calorie_reserve * 0.3);
                mother.calorie_reserve -= transfer;
                mother.add_child(child_id);
                transfer
            } else {
                800.0
            };

            let mut newborn = Human::new(child_id, sex, current_tick)
                .with_location(mother_loc)
                .with_calories(maternal_investment)
                .with_parents(Some(father_id), Some(mother_id));
            
            // Give baby starting attributes with inherited generation
            newborn.attributes = serde_json::json!({
                "lineage": format!("Child of {} and {}", father_id, mother_id),
                "generation": mother_gen + 1
            });

            // Update father
            if let Some(father) = agent_store.get_human_mut(father_id) {
                father.add_child(child_id);
            }

            agent_store.insert_human(newborn);
        }

        // 6. Vertical Cultural Transmission (Parental Education)
        // Living parents pass non-rival knowledge blueprints to adolescent children (ages 10..=22)
        if current_tick.0 % 30 == 0 {
            let mut transmissions = Vec::new();
            for id in &living_ids {
                if let Some(child) = agent_store.get_human(*id) {
                    let age_years = (child.age_ticks as f64) * fractional_years;
                    if child.is_alive() && (10.0..=22.0).contains(&age_years) {
                        let father_id = child.father_id;
                        let mother_id = child.mother_id;
                        let child_id = *id;

                        let mut parent_knowledges = Vec::new();
                        if let Some(f_id) = father_id {
                            if let Some(father) = agent_store.get_human(f_id) {
                                if father.is_alive() {
                                    for (&k, _) in &father.inventory {
                                        if k.is_knowledge() && !child.inventory.contains_key(&k) {
                                            parent_knowledges.push(k);
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(m_id) = mother_id {
                            if let Some(mother) = agent_store.get_human(m_id) {
                                if mother.is_alive() {
                                    for (&k, _) in &mother.inventory {
                                        if k.is_knowledge()
                                            && !child.inventory.contains_key(&k)
                                            && !parent_knowledges.contains(&k)
                                        {
                                            parent_knowledges.push(k);
                                        }
                                    }
                                }
                            }
                        }
                        for k in parent_knowledges {
                            transmissions.push((child_id, k));
                        }
                    }
                }
            }
            for (child_id, k) in transmissions {
                if let Some(child) = agent_store.get_human_mut(child_id) {
                    child.add_item(k, 1);
                }
            }
        }
    }
}

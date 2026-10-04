use crate::core::domain::agent::human::{Human, Sex};
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::{HasLifecycle, SocialActor};
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
        let all_ids = agent_store.all_human_ids();
        let fractional_years = tick_duration.fractional_years();

        // 1. Advance age for all living agents
        for id in &all_ids {
            if let Some(agent) = agent_store.get_human_mut(*id).filter(|a| a.is_alive()) {
                agent.step_age(1);
            }
        }

        // 2. Mortality Check (Gompertz-Makeham hazard model)
        for id in &all_ids {
            if let Some(agent) = agent_store.get_human_mut(*id) {
                if !agent.is_alive() {
                    continue;
                }

                let age_in_years = (agent.age_ticks as f64) * fractional_years;
                
                // Annual mortality probability using Gompertz-Makeham curve
                // Baseline hazard + exponential increase with age
                let annual_mortality = (0.001 + 0.00008 * (1.095_f64).powf(age_in_years)).min(0.999);
                
                // Convert annual probability to per-tick probability: 1 - (1 - P_annual)^fractional_years
                let tick_mortality = 1.0 - (1.0 - annual_mortality).powf(fractional_years);

                if rng.check_probability(tick_mortality) {
                    agent.mark_deceased(current_tick, format!("Natural causes at age {:.1} years", age_in_years));
                }
            }
        }

        // 3. Marriage Matching
        // Collect eligible living unmarried males and females of legal age (>= 18 years)
        let mut eligible_males = Vec::new();
        let mut eligible_females = Vec::new();

        for id in agent_store.all_human_ids() {
            if let Some(agent) = agent_store.get_human(id) {
                let age_years = (agent.age_ticks as f64) * fractional_years;
                if agent.is_alive() && agent.spouse_id.is_none() && (18.0..=65.0).contains(&age_years) {
                    match agent.sex {
                        Sex::Male => eligible_males.push(id),
                        Sex::Female => eligible_females.push(id),
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

        // 4. Childbirth (Reproduction)
        // Check married couples where female is between 18 and 45 years
        let mut births_to_create = Vec::new();

        for id in agent_store.all_human_ids() {
            if let Some(female) = agent_store.get_human(id) {
                let age_years = (female.age_ticks as f64) * fractional_years;
                if female.is_alive()
                    && female.sex == Sex::Female
                    && (18.0..=45.0).contains(&age_years)
                    && let Some(spouse_id) = female.spouse_id
                {
                    let annual_birth_rate: f64 = 0.30;
                    let tick_birth_prob = 1.0_f64 - (1.0_f64 - annual_birth_rate).powf(fractional_years);

                    if rng.check_probability(tick_birth_prob) {
                        let newborn_sex = if rng.check_probability(0.5) {
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

            let mut newborn = Human::new(child_id, sex, current_tick)
                .with_location(mother_loc)
                .with_calories(15000.0)
                .with_parents(Some(father_id), Some(mother_id));
            
            // Give baby starting attributes
            newborn.attributes = serde_json::json!({
                "lineage": format!("Child of {} and {}", father_id, mother_id),
                "generation": 2
            });

            // Update parents
            if let Some(father) = agent_store.get_human_mut(father_id) {
                father.add_child(child_id);
            }
            if let Some(mother) = agent_store.get_human_mut(mother_id) {
                mother.add_child(child_id);
            }

            agent_store.insert_human(newborn);
        }
    }
}

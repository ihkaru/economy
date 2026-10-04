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
use crate::core::ports::statistic_store::StatisticStorePort;
use crate::core::systems::exchange::{
    perform_agent_centric_foraging, perform_autonomous_crafting, perform_trade_and_services,
};

pub struct ExchangeSystem {
    next_trx_id: u64,
    next_instance_id: u64,
}

impl ExchangeSystem {
    pub const NATURE_AGENT_ID: AgentId = AgentId(0);

    pub fn new(initial_trx_id: u64) -> Self {
        Self {
            next_trx_id: initial_trx_id,
            next_instance_id: 1,
        }
    }

    /// Step natural resource harvesting, spontaneous self-learning discoveries, crafting, and multi-item barter
    pub fn step(
        &mut self,
        run_id: &RunId,
        current_tick: Tick,
        agent_store: &mut dyn AgentStorePort,
        env_store: &mut dyn EnvironmentStorePort,
        ledger_store: &mut dyn LedgerStorePort,
        stat_store: &dyn StatisticStorePort,
        rng: &mut dyn RngPort,
    ) {
        let living_agent_ids = agent_store.living_human_ids();
        if living_agent_ids.is_empty() {
            return;
        }

        // 1. Spontaneous Knowledge Discovery (Otodidak / Eureka Breakthroughs by well-fed agents)
        let mut discoveries: Vec<(AgentId, ItemId, &'static str)> = Vec::new();
        for id in &living_agent_ids {
            if let Some(agent) = agent_store.get_human(*id) {
                // Agents with leisure and high energy reserves can experiment and discover ideas
                if agent.calorie_reserve > 10000.0 && agent.days_starving == 0 {
                    // Raft building knowledge discovery (if holding Timber near water)
                    if agent.has_item(ItemId::TIMBER)
                        && !agent.has_item(ItemId::KNOWLEDGE_RAFT_BUILDING)
                        && rng.check_probability(0.02)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_RAFT_BUILDING, "Raft Construction Blueprint"));
                    }
                    // Fish curing knowledge discovery (if holding Fish and Salt)
                    if agent.has_item(ItemId::FISH)
                        && agent.has_item(ItemId::SALT)
                        && !agent.has_item(ItemId::KNOWLEDGE_FISH_CURING)
                        && rng.check_probability(0.03)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_FISH_CURING, "Salting & Fish Curing Preservation"));
                    }
                    // Tool crafting knowledge discovery (if holding Timber)
                    if agent.has_item(ItemId::TIMBER)
                        && !agent.has_item(ItemId::KNOWLEDGE_TOOL_CRAFTING)
                        && rng.check_probability(0.02)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_TOOL_CRAFTING, "Tool Crafting Blueprint"));
                    }
                    // Herbal medicine knowledge discovery (if holding Berries or Herbs)
                    if (agent.has_item(ItemId::BERRIES) || agent.has_item(ItemId::HERBAL_MEDICINE))
                        && !agent.has_item(ItemId::KNOWLEDGE_HERBAL_MEDICINE)
                        && rng.check_probability(0.03)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_HERBAL_MEDICINE, "Herbal Medicine Blueprint"));
                    }
                    // Basket weaving knowledge discovery (if holding Timber/fibers)
                    if agent.has_item(ItemId::TIMBER)
                        && !agent.has_item(ItemId::KNOWLEDGE_BASKET_WEAVING)
                        && rng.check_probability(0.02)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_BASKET_WEAVING, "Basket Weaving Blueprint"));
                    }
                    // Fire-making pyrotechnology discovery (if holding Timber)
                    if agent.has_item(ItemId::TIMBER)
                        && !agent.has_item(ItemId::KNOWLEDGE_FIRE_MAKING)
                        && rng.check_probability(0.02)
                    {
                        discoveries.push((*id, ItemId::KNOWLEDGE_FIRE_MAKING, "Fire-Making Technique"));
                    }
                }
            }
        }

        for (agent_id, knowledge_id, name) in discoveries {
            if let Some(agent) = agent_store.get_human_mut(agent_id) {
                agent.add_item(knowledge_id, 1);
            }

            let item_instance = ItemInstance::new(
                knowledge_id,
                1,
                serde_json::json!({
                    "blueprint": name,
                    "nature": "NonRivalKnowledge",
                }),
            )
            .with_instance_id(ItemInstanceId::new(self.next_instance_id));
            self.next_instance_id += 1;

            let entry = LedgerEntry::single(
                self.next_trx_id,
                run_id.clone(),
                current_tick,
                Self::NATURE_AGENT_ID, // Party A: Nature / Mind / Epiphany
                agent_id,              // Party B: Inventor
                Some(item_instance),
                None,
                serde_json::json!({
                    "transaction_type": "scientific_discovery",
                    "knowledge_name": name,
                    "discovery_mechanism": "trial_and_error_eureka",
                }),
            );
            self.next_trx_id += 1;
            let _ = ledger_store.record(entry);
        }

        // 2. Agent Interaction with Environment: Agent-Centric Optimal Foraging
        perform_agent_centric_foraging(
            run_id,
            current_tick,
            &living_agent_ids,
            agent_store,
            env_store,
            ledger_store,
            rng,
            &mut self.next_trx_id,
            &mut self.next_instance_id,
        );

        // 3. Autonomous Value-Added Manufacturing & Multi-Tier Crafting (Leontief Recipe DAG)
        perform_autonomous_crafting(
            run_id,
            current_tick,
            &living_agent_ids,
            agent_store,
            ledger_store,
            rng,
            &mut self.next_trx_id,
            &mut self.next_instance_id,
        );

        // 4. Inter-Agent Trade (Emergent Bilateral Barter, Medical Services & Apprenticeship)
        if living_agent_ids.len() >= 2 {
            let idx_a = rng.gen_range_u64(0, living_agent_ids.len() as u64) as usize;
            let mut idx_b = rng.gen_range_u64(0, (living_agent_ids.len() - 1) as u64) as usize;
            if idx_b >= idx_a {
                idx_b += 1;
            }

            let agent_a_id = living_agent_ids[idx_a];
            let agent_b_id = living_agent_ids[idx_b];

            // Spatial check: Calculate distance between Agent A and Agent B
            let (_dist, can_reach) = {
                let a = agent_store.get_human(agent_a_id);
                let b = agent_store.get_human(agent_b_id);
                if let (Some(a), Some(b)) = (a, b) {
                    let d = a.location.euclidean_distance(&b.location);
                    let a_has_vessel = a.has_item(ItemId::RAFT);
                    let b_has_vessel = b.has_item(ItemId::RAFT);
                    let reachable = d <= 10.0 || (a_has_vessel || b_has_vessel);
                    (d, reachable)
                } else {
                    (0.0, false)
                }
            };

            if can_reach {
                // Maritime journey wear-and-tear on raft
                if _dist > 10.0 && rng.check_probability(0.01) {
                    if let Some(agent_a) = agent_store.get_human_mut(agent_a_id) {
                        if agent_a.has_item(ItemId::RAFT) {
                            let _ = agent_a.remove_item(ItemId::RAFT, 1);
                        }
                    }
                }

                perform_trade_and_services(
                    run_id,
                    current_tick,
                    agent_a_id,
                    agent_b_id,
                    agent_store,
                    ledger_store,
                    stat_store,
                    &mut self.next_trx_id,
                    &mut self.next_instance_id,
                );
            }
        }
    }
}

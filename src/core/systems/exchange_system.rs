use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::{EconomicActor, HasLifecycle};
use crate::core::domain::item::id::{ItemId, ItemInstanceId};
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::time::{RunId, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::rng_port::RngPort;
use crate::core::ports::statistic_store::StatisticStorePort;
use crate::core::systems::exchange::perform_trade_and_services;

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
        let living_agent_ids: Vec<AgentId> = agent_store
            .all_human_ids()
            .into_iter()
            .filter(|id| {
                agent_store
                    .get_human(*id)
                    .map(|a| a.is_alive())
                    .unwrap_or(false)
            })
            .collect();

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

        // 2. Agent Interaction with Environment: Resource Harvesting
        for node in env_store.nodes_mut() {
            if node.current_stock == 0 {
                continue;
            }

            // Find living agents within reachable foraging distance (<= 5 cells)
            let reachable_agents: Vec<AgentId> = living_agent_ids
                .iter()
                .filter(|id| {
                    agent_store
                        .get_human(**id)
                        .map(|a| a.location.euclidean_distance(&node.location) <= 5.0)
                        .unwrap_or(false)
                })
                .copied()
                .collect();

            if reachable_agents.is_empty() {
                continue;
            }

            // Pick a random reachable agent
            let agent_idx = rng.gen_range_u64(0, reachable_agents.len() as u64) as usize;
            let agent_id = reachable_agents[agent_idx];

            // Determine capital tool efficiency multiplier
            let tool_efficiency = if let Some(agent) = agent_store.get_human(agent_id) {
                if (node.item_id == ItemId::TIMBER && agent.has_item(ItemId::STONE_AXE))
                    || (node.item_id == ItemId::FISH && agent.has_item(ItemId::FISHING_NET))
                {
                    3.0 // 3x harvest efficiency with appropriate capital tool
                } else {
                    1.0
                }
            } else {
                1.0
            };

            let harvest_attempt = rng.gen_range_u64(1, 5) as u32;
            let actual_harvested = node.harvest(harvest_attempt, tool_efficiency);

            if actual_harvested > 0 {
                if let Some(agent) = agent_store.get_human_mut(agent_id) {
                    agent.add_item(node.item_id, actual_harvested);
                }

                // Record transaction in Ultimate Ledger: Nature -> Agent
                let item_instance = ItemInstance::new(
                    node.item_id,
                    actual_harvested,
                    serde_json::json!({
                        "source_node": node.name,
                        "resource_id": node.id,
                        "tool_multiplier": tool_efficiency,
                        "node_maturity": node.maturity,
                    }),
                )
                .with_instance_id(ItemInstanceId::new(self.next_instance_id));
                self.next_instance_id += 1;

                let entry = LedgerEntry::single(
                    self.next_trx_id,
                    run_id.clone(),
                    current_tick,
                    Self::NATURE_AGENT_ID, // Party A: Nature
                    agent_id,              // Party B: Harvester
                    Some(item_instance),
                    None,
                    serde_json::json!({
                        "transaction_type": "natural_resource_harvest",
                        "node_name": node.name,
                        "tool_multiplier": tool_efficiency,
                        "node_maturity": node.maturity,
                        "remaining_node_stock": node.current_stock
                    }),
                );
                self.next_trx_id += 1;

                let _ = ledger_store.record(entry);

                // Knowledge-gated Autonomous Crafting Emergence (Roundabout Capital Production):
                let mut crafted_items: Vec<(ItemId, u32, u32, &'static str)> = Vec::new();
                if let Some(agent) = agent_store.get_human(agent_id) {
                    let timber_stock = agent.inventory.get(&ItemId::TIMBER).copied().unwrap_or(0);
                    // Craft Stone Axe if agent has tool knowledge, no axe, and >= 5 timber
                    if agent.has_item(ItemId::KNOWLEDGE_TOOL_CRAFTING)
                        && !agent.has_item(ItemId::STONE_AXE)
                        && timber_stock >= 5
                    {
                        crafted_items.push((ItemId::STONE_AXE, 1, 5, "Stone Axe"));
                    }
                    // Craft Fishing Net if agent has tool knowledge, no net, and >= 4 timber
                    else if agent.has_item(ItemId::KNOWLEDGE_TOOL_CRAFTING)
                        && !agent.has_item(ItemId::FISHING_NET)
                        && timber_stock >= 4
                    {
                        crafted_items.push((ItemId::FISHING_NET, 1, 4, "Fishing Net"));
                    }
                    // Craft Raft if agent knows raft building, has no raft, and >= 10 timber
                    else if agent.has_item(ItemId::KNOWLEDGE_RAFT_BUILDING)
                        && !agent.has_item(ItemId::RAFT)
                        && timber_stock >= 10
                    {
                        crafted_items.push((ItemId::RAFT, 1, 10, "Maritime Raft"));
                    }
                }

                for (tool_id, tool_qty, cost_timber, tool_name) in crafted_items {
                    if let Some(agent) = agent_store.get_human_mut(agent_id) {
                        let _ = agent.remove_item(ItemId::TIMBER, cost_timber);
                        agent.add_item(tool_id, tool_qty);
                    }

                    let spent_timber = ItemInstance::new(
                        ItemId::TIMBER,
                        cost_timber,
                        serde_json::json!({"nature": "ConsumedRawMaterial"}),
                    )
                    .with_instance_id(ItemInstanceId::new(self.next_instance_id));
                    self.next_instance_id += 1;

                    let produced_tool = ItemInstance::new(
                        tool_id,
                        tool_qty,
                        serde_json::json!({
                            "nature": "CapitalGood",
                            "tool_name": tool_name,
                        }),
                    )
                    .with_instance_id(ItemInstanceId::new(self.next_instance_id));
                    self.next_instance_id += 1;

                    let entry = LedgerEntry::new(
                        self.next_trx_id,
                        run_id.clone(),
                        current_tick,
                        agent_id,
                        agent_id, // Internal transformation / production ledger
                        vec![spent_timber],
                        vec![produced_tool],
                        serde_json::json!({
                            "transaction_type": "capital_tool_production",
                            "tool_crafted": tool_name,
                            "timber_invested": cost_timber,
                        }),
                    );
                    self.next_trx_id += 1;
                    let _ = ledger_store.record(entry);
                }
            }
        }

        // 3. Inter-Agent Trade (Emergent Bilateral Barter & Knowledge Education Services)
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

use std::collections::BTreeMap;
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::{EconomicActor, HasLifecycle};
use crate::core::domain::item::id::{ItemId, ItemInstanceId};
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::domain::spatial::coordinate::GeoCoordinate;
use crate::core::domain::statistic::table::TableCell;
use crate::core::domain::time::{RunId, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::rng_port::RngPort;
use crate::core::ports::statistic_store::StatisticStorePort;

/// Evaluates an agent's subjective marginal utility for 1 unit of an item
fn evaluate_marginal_utility(item_id: ItemId, calorie_reserve: f64, current_stock: u32) -> f64 {
    let diminishing = 1.0 / (1.0 + current_stock as f64);
    let base = match item_id {
        ItemId::GRAIN => {
            let urgency = (10000.0 / (calorie_reserve + 500.0)).clamp(0.5, 10.0);
            70.0 * urgency
        }
        ItemId::FISH => {
            let urgency = (8000.0 / (calorie_reserve + 500.0)).clamp(0.5, 8.0);
            50.0 * urgency
        }
        ItemId::BERRIES => {
            let urgency = (6000.0 / (calorie_reserve + 500.0)).clamp(0.5, 6.0);
            30.0 * urgency
        }
        ItemId::TIMBER => 35.0,
        ItemId::SALT => 55.0, // High durability, good preservative, natural medium of exchange
        ItemId::RAFT => 150.0,
        ItemId::STONE_AXE => 120.0,
        ItemId::FISHING_NET => 110.0,
        ItemId::SHELLS => 40.0,
        // Non-rival Knowledge blueprints
        ItemId::KNOWLEDGE_RAFT_BUILDING => 200.0,
        ItemId::KNOWLEDGE_TOOL_CRAFTING => 180.0,
        ItemId::KNOWLEDGE_FISH_CURING => 120.0,
        ItemId::KNOWLEDGE_FIRE_MAKING => 100.0,
        // Institutional Permits
        ItemId::PERMIT_FISHING_RIGHT => 80.0,
        ItemId::PERMIT_FORESTRY_RIGHT => 80.0,
        _ => 20.0,
    };
    base * diminishing
}

/// Query the latest published commodity table to derive market scarcity expectations (Hayekian Market Intelligence)
fn derive_market_scarcity_multiplier(stat_store: &dyn StatisticStorePort, item_id: ItemId) -> f64 {
    if let Some(release) = stat_store.get_latest_table("TAB_COMM_01") {
        for row in &release.table.rows {
            if let Some(TableCell::Integer(id_val)) = row.cells.first() {
                if *id_val as u64 == item_id.as_u64() {
                    // Check nature stock in cell 4
                    if let Some(TableCell::Integer(nature_qty)) = row.cells.get(4) {
                        if *nature_qty < 250 {
                            return 1.45; // Critical scarcity expectation: high speculative valuation
                        } else if *nature_qty < 1000 {
                            return 1.20; // Moderate scarcity expectation
                        } else if *nature_qty > 5000 {
                            return 0.85; // High abundance discount
                        }
                    }
                }
            }
        }
    }
    1.0
}

/// Evaluates whether an agent has cognitive/spatial access to the latest published market statistics (Bounded Local Rationality)
fn agent_has_market_access(agent_id: AgentId, agent_store: &dyn AgentStorePort) -> bool {
    if let Some(a) = agent_store.get_human(agent_id) {
        // Proximity to the central settlement bulletin post (15, 25 within 8 cells)
        let near_bulletin = a.location.euclidean_distance(&GeoCoordinate::new(15, 25)) <= 8.0;
        // Or literacy / education (knows tool crafting or raft building)
        let educated = a.has_item(ItemId::KNOWLEDGE_TOOL_CRAFTING) || a.has_item(ItemId::KNOWLEDGE_RAFT_BUILDING);
        near_bulletin || educated
    } else {
        false
    }
}

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
                let (a_cal, a_inv) = agent_store
                    .get_human(agent_a_id)
                    .map(|a| (a.calorie_reserve, a.inventory.clone()))
                    .unwrap_or((0.0, BTreeMap::new()));
                let (b_cal, b_inv) = agent_store
                    .get_human(agent_b_id)
                    .map(|b| (b.calorie_reserve, b.inventory.clone()))
                    .unwrap_or((0.0, BTreeMap::new()));

                // Case A: Knowledge Service Trade (Apprenticeship/Education)
                // If Agent A has knowledge that Agent B lacks, A can teach B in exchange for a bundle of physical goods
                let mut knowledge_trade_occurred = false;
                for k_id in [
                    ItemId::KNOWLEDGE_RAFT_BUILDING,
                    ItemId::KNOWLEDGE_FISH_CURING,
                    ItemId::KNOWLEDGE_TOOL_CRAFTING,
                ] {
                    if a_inv.contains_key(&k_id) && !b_inv.contains_key(&k_id) {
                        // Check if B has edible food to pay the teacher
                        let b_food = [ItemId::GRAIN, ItemId::FISH, ItemId::BERRIES]
                            .into_iter()
                            .find(|f| b_inv.get(f).copied().unwrap_or(0) >= 2);

                        if let Some(payment_food) = b_food {
                            // Execute knowledge transfer:
                            // A teaches B: A does NOT lose knowledge (non-rival!), B gains knowledge.
                            // B pays A: B transfers 2 units of food to A.
                            if let Some(agent_a) = agent_store.get_human_mut(agent_a_id) {
                                agent_a.add_item(payment_food, 2);
                            }
                            if let Some(agent_b) = agent_store.get_human_mut(agent_b_id) {
                                let _ = agent_b.remove_item(payment_food, 2);
                                agent_b.add_item(k_id, 1);
                            }

                            let inst_k = ItemInstance::new(
                                k_id,
                                1,
                                serde_json::json!({"nature": "NonRivalKnowledge"}),
                            ).with_instance_id(ItemInstanceId::new(self.next_instance_id));
                            self.next_instance_id += 1;

                            let inst_food = ItemInstance::new(
                                payment_food,
                                2,
                                serde_json::json!({"nature": "RivalPhysical"}),
                            ).with_instance_id(ItemInstanceId::new(self.next_instance_id));
                            self.next_instance_id += 1;

                            let entry = LedgerEntry::new(
                                self.next_trx_id,
                                run_id.clone(),
                                current_tick,
                                agent_a_id,
                                agent_b_id,
                                vec![inst_k],
                                vec![inst_food],
                                serde_json::json!({
                                    "transaction_type": "knowledge_service_trade",
                                    "knowledge_item_id": k_id.0,
                                    "tuition_paid_item_id": payment_food.0,
                                    "tuition_paid_qty": 2,
                                }),
                            );
                            self.next_trx_id += 1;
                            let _ = ledger_store.record(entry);

                            knowledge_trade_occurred = true;
                            break;
                        }
                    }
                }

                // Case B: Bilateral Physical Goods Barter (if no knowledge trade occurred)
                // Incorporating Hayekian Bounded Local Rationality: Informed agents who query the statistical table adjust valuations
                if !knowledge_trade_occurred {
                    let a_informed = agent_has_market_access(agent_a_id, agent_store);
                    let b_informed = agent_has_market_access(agent_b_id, agent_store);

                    let mult_a = |id: ItemId| if a_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };
                    let mult_b = |id: ItemId| if b_informed { derive_market_scarcity_multiplier(stat_store, id) } else { 1.0 };

                    let a_offer = a_inv
                        .iter()
                        .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
                        .min_by(|(id1, q1), (id2, q2)| {
                            let u1 = evaluate_marginal_utility(**id1, a_cal, **q1) * mult_a(**id1);
                            let u2 = evaluate_marginal_utility(**id2, a_cal, **q2) * mult_a(**id2);
                            u1.partial_cmp(&u2).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|(id, _)| *id);

                    let b_offer = b_inv
                        .iter()
                        .filter(|(id, qty)| **qty > 0 && !id.is_knowledge())
                        .min_by(|(id1, q1), (id2, q2)| {
                            let u1 = evaluate_marginal_utility(**id1, b_cal, **q1) * mult_b(**id1);
                            let u2 = evaluate_marginal_utility(**id2, b_cal, **q2) * mult_b(**id2);
                            u1.partial_cmp(&u2).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|(id, _)| *id);

                    if let (Some(item_a), Some(item_b)) = (a_offer, b_offer)
                        && item_a != item_b
                    {
                        let a_stock_a = a_inv.get(&item_a).copied().unwrap_or(0);
                        let a_stock_b = a_inv.get(&item_b).copied().unwrap_or(0);
                        let b_stock_a = b_inv.get(&item_a).copied().unwrap_or(0);
                        let b_stock_b = b_inv.get(&item_b).copied().unwrap_or(0);

                        let u_a_gives = evaluate_marginal_utility(item_a, a_cal, a_stock_a) * mult_a(item_a);
                        let u_a_receives = evaluate_marginal_utility(item_b, a_cal, a_stock_b) * mult_a(item_b);
                        let u_b_gives = evaluate_marginal_utility(item_b, b_cal, b_stock_b) * mult_b(item_b);
                        let u_b_receives = evaluate_marginal_utility(item_a, b_cal, b_stock_a) * mult_b(item_a);

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
                            ).with_instance_id(ItemInstanceId::new(self.next_instance_id));
                            self.next_instance_id += 1;

                            let instance_b = ItemInstance::new(
                                item_b,
                                1,
                                serde_json::json!({"source_agent": agent_b_id.0}),
                            ).with_instance_id(ItemInstanceId::new(self.next_instance_id));
                            self.next_instance_id += 1;

                            let entry = LedgerEntry::new(
                                self.next_trx_id,
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
                                    "information_asymmetry": a_informed != b_informed,
                                }),
                            );
                            self.next_trx_id += 1;
                            let _ = ledger_store.record(entry);
                        }
                    }
                }
            }
        }
    }
}

use crate::core::domain::agent::human::Human;
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::EconomicActor;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::spatial::coordinate::GeoCoordinate;
use crate::core::domain::statistic::table::TableCell;
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::statistic_store::StatisticStorePort;

/// Evaluates an agent's subjective marginal utility for 1 unit of an item
pub fn evaluate_marginal_utility(item_id: ItemId, calorie_reserve: f64, current_stock: u32) -> f64 {
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
        ItemId::RAW_MEAT => {
            let urgency = (8500.0 / (calorie_reserve + 500.0)).clamp(0.5, 8.5);
            55.0 * urgency
        }
        ItemId::BERRIES => {
            let urgency = (6000.0 / (calorie_reserve + 500.0)).clamp(0.5, 6.0);
            30.0 * urgency
        }
        ItemId::CURED_FISH => {
            let urgency = (9000.0 / (calorie_reserve + 500.0)).clamp(0.5, 9.0);
            65.0 * urgency
        }
        ItemId::SMOKED_FISH => {
            let urgency = (8500.0 / (calorie_reserve + 500.0)).clamp(0.5, 8.5);
            60.0 * urgency
        }
        ItemId::SMOKED_MEAT => {
            let urgency = (9000.0 / (calorie_reserve + 500.0)).clamp(0.5, 9.0);
            65.0 * urgency
        }
        ItemId::CURED_MEAT => {
            let urgency = (9000.0 / (calorie_reserve + 500.0)).clamp(0.5, 9.0);
            70.0 * urgency
        }
        ItemId::DRIED_BERRIES => {
            let urgency = (7000.0 / (calorie_reserve + 500.0)).clamp(0.5, 7.0);
            45.0 * urgency
        }
        ItemId::TIMBER => 35.0,
        ItemId::STONE => 30.0,
        ItemId::RAW_HIDE => 45.0,
        ItemId::LEATHER_CLOTHING => 180.0,
        ItemId::HUNTING_SPEAR => 130.0,
        ItemId::SALT => 55.0, // High durability, good preservative, natural medium of exchange
        ItemId::RAFT => 150.0,
        ItemId::STONE_AXE => 120.0,
        ItemId::FISHING_NET => 110.0,
        ItemId::SHELLS => 40.0,
        ItemId::CLAY => 25.0,
        ItemId::POTTERY_JAR => 160.0,
        // Non-rival Knowledge blueprints
        ItemId::KNOWLEDGE_RAFT_BUILDING => 200.0,
        ItemId::KNOWLEDGE_TOOL_CRAFTING => 180.0,
        ItemId::KNOWLEDGE_FISH_CURING => 120.0,
        ItemId::KNOWLEDGE_FIRE_MAKING => 100.0,
        ItemId::KNOWLEDGE_POTTERY_MAKING => 150.0,
        ItemId::KNOWLEDGE_LEATHER_WORKING => 160.0,
        // Institutional Permits
        ItemId::PERMIT_FISHING_RIGHT => 80.0,
        ItemId::PERMIT_FORESTRY_RIGHT => 80.0,
        _ => 20.0,
    };
    base * diminishing
}

/// Carl Menger's Saleability (Absatzfähigkeit): durable/universal goods carry a liquidity premium
pub fn evaluate_liquidity_premium(id: ItemId) -> f64 {
    match id {
        ItemId::WAREHOUSE_RECEIPT => 1.75, // Highest saleability: warehouse certificate
        ItemId::CLAY_TABLET => 1.60,       // High saleability: promissory debt token
        ItemId::SHELLS | ItemId::SALT => 1.40, // Commodity currency premium
        ItemId::GRAIN => 1.15,             // Staple currency backup
        _ => 1.0,
    }
}

/// Updates an agent's internal personal stats with lagged market observation (enforcing observation lag, never real-time)
pub fn update_agent_market_observation(agent: &mut Human, stat_store: &dyn StatisticStorePort, current_tick: u64) {
    let lag = current_tick.saturating_sub(agent.personal_stats.last_observation_tick);
    // Agents only refresh their observation if at least 15 ticks have elapsed since last visit
    if lag >= 15 {
        if let Some(release) = stat_store.get_latest_table("TAB_COMM_01") {
            let observation_lag = 7.max(current_tick.saturating_sub(release.release_tick.0));
            for row in &release.table.rows {
                if let (Some(TableCell::Integer(id_val)), Some(TableCell::Integer(nature_qty))) = (row.cells.first(), row.cells.get(4)) {
                    let item_id = ItemId::new(*id_val as u64);
                    let signal = if *nature_qty < 250 {
                        1.45
                    } else if *nature_qty < 1000 {
                        1.20
                    } else if *nature_qty > 5000 {
                        0.85
                    } else {
                        1.0
                    };
                    agent.personal_stats.observe_market_with_lag(item_id, signal, current_tick, observation_lag);
                }
            }
        }
    }
}

/// Query the latest published commodity table to derive market scarcity expectations (Hayekian Market Intelligence)
pub fn derive_market_scarcity_multiplier(stat_store: &dyn StatisticStorePort, item_id: ItemId) -> f64 {
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
pub fn agent_has_market_access(agent_id: AgentId, agent_store: &dyn AgentStorePort) -> bool {
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

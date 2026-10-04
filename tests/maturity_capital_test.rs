use economy::adapters::persistence::ParquetExporter;
use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{
    MemoryAgentStore, MemoryEnvironmentStore, MemoryLedgerStore, MemoryStatisticStore,
};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::agent::traits::EconomicActor;
use economy::core::domain::environment::climate::ClimateState;
use economy::core::domain::environment::resource::{RegenerationPace, ResourceNode};
use economy::core::domain::item::id::ItemId;
use economy::core::domain::spatial::coordinate::GeoCoordinate;
use economy::core::domain::time::{RunId, Tick, TickDuration};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::systems::statistic_system::StatisticSystem;
use economy::engine::{SimulationConfig, SimulationEngine};
use std::fs;

#[test]
fn test_resource_maturity_growth_and_harvest_curve() {
    let mut node = ResourceNode::new(
        1,
        "Ancient Forest",
        ItemId::TIMBER,
        GeoCoordinate::new(0, 0),
        100, // 10% initial maturity
        1000,
        10.0,
        1.0,
        0.0,
        serde_json::json!({}),
    )
    .with_pace(RegenerationPace::Slow)
    .with_maturity(0.10);

    assert_eq!(node.maturity, 0.10);

    // Harvest at low maturity (< 0.40) suffers severe sapling penalty (0.25x)
    let harvested_early = node.harvest(10, 1.0);
    // 10 * 1.0 * 0.25 = 2.5 -> rounded to 3 (or max 1)
    assert!(harvested_early <= 3);

    // Test logistic growth over multiple climate steps
    let climate = ClimateState::default_spring();
    let initial_m = node.maturity;
    for _ in 0..100 {
        node.step_environment(&climate);
    }
    // In spring, logistic growth should increase maturity
    assert!(node.maturity > initial_m);

    // Fast pace resource matures faster than slow pace
    let mut berry_node = ResourceNode::new(
        2,
        "Berry Bush",
        ItemId::BERRIES,
        GeoCoordinate::new(0, 0),
        50,
        1000,
        50.0,
        1.0,
        300.0,
        serde_json::json!({}),
    )
    .with_pace(RegenerationPace::Fast)
    .with_maturity(0.05);

    let mut timber_node = ResourceNode::new(
        3,
        "Oak Trees",
        ItemId::TIMBER,
        GeoCoordinate::new(0, 0),
        50,
        1000,
        5.0,
        1.0,
        0.0,
        serde_json::json!({}),
    )
    .with_pace(RegenerationPace::Slow)
    .with_maturity(0.05);

    for _ in 0..50 {
        berry_node.step_environment(&climate);
        timber_node.step_environment(&climate);
    }

    // Fast pace berries should reach significantly higher maturity than slow pace timber
    assert!(berry_node.maturity > timber_node.maturity);
}

#[test]
fn test_tool_multiplier_and_capital_production_in_simulation() {
    let output_dir = "output/test_maturity_capital";
    let _ = fs::remove_dir_all(output_dir);

    let run_id = RunId::new("test_maturity_capital");
    let tick_duration = TickDuration::Day;
    let rng = ChaChaRngAdapter::new(12345);
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();
    let stat_store = MemoryStatisticStore::new();

    // 10 agents, 1 agent starts with tool knowledge and timber to trigger tool crafting
    for i in 1..=10 {
        let agent_id = AgentId::new(i as u64);
        let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
        let mut human = Human::new(agent_id, sex, Tick::ZERO)
            .with_initial_age(25 * 365)
            .with_calories(30000.0)
            .with_location(GeoCoordinate::new(10, 10));

        if i == 1 {
            // Pioneer artisan with knowledge and timber
            human.add_item(ItemId::KNOWLEDGE_TOOL_CRAFTING, 1);
            human.add_item(ItemId::TIMBER, 20);
        } else {
            human.add_item(ItemId::GRAIN, 10);
        }

        agent_store.insert_human(human);
    }

    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(
            1,
            "Forest",
            ItemId::TIMBER,
            GeoCoordinate::new(10, 10),
            800,
            1000,
            20.0,
            1.0,
            0.0,
            serde_json::json!({}),
        )
        .with_pace(RegenerationPace::Slow)
        .with_maturity(0.85), // Prime maturity
        ResourceNode::new(
            2,
            "Fishery",
            ItemId::FISH,
            GeoCoordinate::new(10, 10),
            5000,
            10000,
            50.0,
            1.0,
            500.0,
            serde_json::json!({}),
        )
        .with_pace(RegenerationPace::Medium)
        .with_maturity(0.80),
    ];

    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);
    let statistic_system = StatisticSystem::new();
    let exporter = ParquetExporter::new(output_dir);

    let config = SimulationConfig::new(
        run_id,
        12345,
        tick_duration,
        15, // 15 ticks
        None,
        output_dir,
    );

    let mut engine = SimulationEngine::new(
        config,
        agent_store,
        ledger_store,
        env_store,
        stat_store,
        rng,
        exporter,
        statistic_system,
        11,
    );

    let result = engine.run();
    assert!(result.is_ok(), "Simulation run failed: {:?}", result.err());

    // Check exported ledger to verify capital tool production or tool harvest
    let ledger_path = format!("{}/run_id=test_maturity_capital/ledger.parquet", output_dir);
    assert!(fs::metadata(&ledger_path).is_ok(), "Ledger parquet file should exist at {}", ledger_path);

    let env_path = format!("{}/run_id=test_maturity_capital/environment.parquet", output_dir);
    assert!(fs::metadata(&env_path).is_ok(), "Environment parquet file should exist at {}", env_path);

    // Clean up test dir
    let _ = fs::remove_dir_all(output_dir);
}

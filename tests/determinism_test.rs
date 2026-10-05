use economy::adapters::persistence::ParquetExporter;
use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{
    MemoryAgentStore, MemoryEnvironmentStore, MemoryLedgerStore, MemoryStatisticStore,
};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::environment::climate::ClimateState;
use economy::core::domain::environment::resource::ResourceNode;
use economy::core::domain::item::id::ItemId;
use economy::core::domain::statistic::access::AccessRequirement;
use economy::core::domain::statistic::calculator::PopulationDemographyCalculator;
use economy::core::domain::statistic::definition::StatisticDefinition;
use economy::core::domain::statistic::schedule::ReleaseSchedule;
use economy::core::domain::time::{RunId, Tick, TickDuration};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::systems::statistic_system::StatisticSystem;
use economy::engine::{SimulationConfig, SimulationEngine};
use std::fs;

fn setup_and_run(run_id_str: &str, seed: u64, total_ticks: u64, output_dir: &str) -> (usize, usize, usize, usize) {
    let run_id = RunId::new(run_id_str);
    let tick_duration = TickDuration::Day;

    let rng = ChaChaRngAdapter::new(seed);
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();
    let stat_store = MemoryStatisticStore::new();

    // 20 initial agents
    for i in 1..=20 {
        let agent_id = AgentId::new(i as u64);
        let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
        let human = Human::new(agent_id, sex, Tick::ZERO)
            .with_initial_age(20 * 365) // 20 years in days
            .with_item(ItemId::new(103), 5);
        agent_store.insert_human(human);
    }

    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(
            1,
            "Forest",
            economy::core::domain::item::id::ItemId::TIMBER,
            economy::core::domain::spatial::GeoCoordinate::new(0, 0),
            500,
            1000,
            10.0,
            2.0,
            0.0,
            serde_json::json!({"type": "wood"}),
        ),
        ResourceNode::new(
            2,
            "Fishery",
            economy::core::domain::item::id::ItemId::FISH,
            economy::core::domain::spatial::GeoCoordinate::new(0, 0),
            300,
            600,
            8.0,
            1.5,
            500.0,
            serde_json::json!({"type": "fish"}),
        ),
    ];
    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);
    let mut statistic_system = StatisticSystem::new();
    statistic_system.register(StatisticDefinition::new(
        "DAILY_POP",
        "Daily Population Release",
        ReleaseSchedule::EveryDays(1),
        AccessRequirement::Public,
        PopulationDemographyCalculator,
    ));

    let exporter = ParquetExporter::new(output_dir);

    let config = SimulationConfig::new(
        run_id,
        seed,
        tick_duration,
        total_ticks,
        None, // Uncapped speed
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
        21,
    );

    let summary = engine.run().expect("Simulation run failed");
    (
        summary.living_agents,
        summary.total_agents,
        summary.total_ledger_transactions,
        summary.total_statistical_releases,
    )
}

#[test]
fn test_exact_determinism_across_identical_seeds() {
    let temp_dir_1 = "target/test_output_1";
    let temp_dir_2 = "target/test_output_2";
    let _ = fs::remove_dir_all(temp_dir_1);
    let _ = fs::remove_dir_all(temp_dir_2);

    let seed = 99999;
    let ticks = 180; // 180 days

    let run_1 = setup_and_run("run-alpha", seed, ticks, temp_dir_1);
    let run_2 = setup_and_run("run-beta", seed, ticks, temp_dir_2);

    // Living agents, total agents (births/deaths), transactions, and statistical publications must be 100% identical!
    assert_eq!(run_1.0, run_2.0, "Living agent count must match identically");
    assert_eq!(run_1.1, run_2.1, "Total agent count must match identically");
    assert_eq!(run_1.2, run_2.2, "Ledger transaction count must match identically");
    assert_eq!(run_1.3, run_2.3, "Statistical releases count must match identically");
    assert_eq!(run_1.3, 180, "Daily publication for 180 ticks must equal 180 releases");

    // Check Parquet files exist
    assert!(fs::metadata(format!("{}/run_id=run-alpha/ledger.parquet", temp_dir_1)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-alpha/agents.parquet", temp_dir_1)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-alpha/environment.parquet", temp_dir_1)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-alpha/statistics.parquet", temp_dir_1)).is_ok());

    assert!(fs::metadata(format!("{}/run_id=run-beta/ledger.parquet", temp_dir_2)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-beta/agents.parquet", temp_dir_2)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-beta/environment.parquet", temp_dir_2)).is_ok());
    assert!(fs::metadata(format!("{}/run_id=run-beta/statistics.parquet", temp_dir_2)).is_ok());

    // Clean up
    let _ = fs::remove_dir_all(temp_dir_1);
    let _ = fs::remove_dir_all(temp_dir_2);
}

#[test]
fn test_different_seeds_produce_divergence() {
    let temp_dir = "target/test_output_divergence";
    let _ = fs::remove_dir_all(temp_dir);

    let run_a = setup_and_run("run-a", 11111, 200, temp_dir);
    let run_b = setup_and_run("run-b", 88888, 200, temp_dir);

    // Different seeds should produce natural statistical divergence in transactions or population
    assert!(run_a.2 != run_b.2 || run_a.0 != run_b.0, "Different seeds should produce divergent outcomes");

    let _ = fs::remove_dir_all(temp_dir);
}

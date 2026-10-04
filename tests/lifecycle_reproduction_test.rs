use economy::adapters::persistence::ParquetExporter;
use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{MemoryAgentStore, MemoryEnvironmentStore, MemoryLedgerStore, MemoryStatisticStore};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::environment::climate::ClimateState;
use economy::core::domain::environment::resource::{RegenerationPace, ResourceNode};
use economy::core::domain::item::id::ItemId;
use economy::core::domain::spatial::GeoCoordinate;
use economy::core::domain::statistic::access::AccessRequirement;
use economy::core::domain::statistic::calculator::PopulationDemographyCalculator;
use economy::core::domain::statistic::definition::StatisticDefinition;
use economy::core::domain::statistic::schedule::ReleaseSchedule;
use economy::core::domain::time::{RunId, Tick, TickDuration};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::systems::statistic_system::StatisticSystem;
use economy::engine::{SimulationConfig, SimulationEngine};

#[test]
fn test_debug_reproduction_engine() {
    let run_id = RunId::new("debug_rep_engine");
    let tick_duration = TickDuration::Day;
    let seed = 42;
    let initial_agents = 50;

    let rng_adapter = ChaChaRngAdapter::new(seed);
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();
    let stat_store = MemoryStatisticStore::new();

    let hours_per_tick = tick_duration.approximate_hours();
    let ticks_per_year = (24.0 * 365.0) / hours_per_tick;

    for i in 1..=initial_agents {
        let agent_id = AgentId::new(i as u64);
        let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
        let initial_age_years = 20.0 + ((i % 11) as f64);
        let initial_age_ticks = (initial_age_years * ticks_per_year).round() as u64;

        let mut human = Human::new(agent_id, sex, Tick::ZERO)
            .with_initial_age(initial_age_ticks)
            .with_item(ItemId::GRAIN, 5)
            .with_item(ItemId::TIMBER, 2)
            .with_location(GeoCoordinate::new(15, 25))
            .with_calories(25000.0);

        human.attributes = serde_json::json!({
            "lineage": "Pioneer Settler",
            "generation": 1
        });

        agent_store.insert_human(human);
    }

    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(1, "Oak", ItemId::TIMBER, GeoCoordinate::new(15, 28), 500, 1000, 10.0, 2.0, 0.0, serde_json::json!({})).with_pace(RegenerationPace::Slow),
        ResourceNode::new(2, "Fish", ItemId::FISH, GeoCoordinate::new(15, 25), 2500, 10000, 80.0, 2.0, 500.0, serde_json::json!({})).with_pace(RegenerationPace::Medium),
        ResourceNode::new(3, "Grain", ItemId::GRAIN, GeoCoordinate::new(16, 24), 4000, 20000, 120.0, 3.0, 800.0, serde_json::json!({})).with_pace(RegenerationPace::Medium),
        ResourceNode::new(4, "Berry", ItemId::BERRIES, GeoCoordinate::new(14, 26), 1500, 5000, 50.0, 1.5, 300.0, serde_json::json!({})).with_pace(RegenerationPace::Fast),
    ];
    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);
    let mut statistic_system = StatisticSystem::new();
    statistic_system.register(StatisticDefinition::new("POP_DEMO", "Demography", ReleaseSchedule::EveryDays(1), AccessRequirement::Public, PopulationDemographyCalculator));

    let exporter = ParquetExporter::new("output/test_scratch");
    // Run for 10 years (3,650 ticks)
    let config = SimulationConfig::new(run_id, seed, tick_duration, 365 * 10, None, "output/test_scratch");

    let mut engine = SimulationEngine::new(config, agent_store, ledger_store, env_store, stat_store, rng_adapter, exporter, statistic_system, 51);

    let _ = std::fs::remove_dir_all("output/test_scratch");

    let summary = engine.run().unwrap();
    println!("Engine 10-year run finished: Total agents: {}, Living: {}", summary.total_agents, summary.living_agents);
    assert!(summary.total_agents > 50, "Expected new births to increase total agents above 50! Found {}", summary.total_agents);

    let _ = std::fs::remove_dir_all("output/test_scratch");
}


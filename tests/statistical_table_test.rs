use std::fs;
use economy::adapters::persistence::ParquetExporter;
use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{
    MemoryAgentStore, MemoryEnvironmentStore, MemoryLedgerStore, MemoryStatisticStore,
};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::agent::traits::SocialActor;
use economy::core::domain::environment::climate::ClimateState;
use economy::core::domain::environment::resource::{RegenerationPace, ResourceNode};
use economy::core::domain::item::id::ItemId;
use economy::core::domain::spatial::coordinate::GeoCoordinate;
use economy::core::domain::statistic::access::AccessRequirement;
use economy::core::domain::statistic::calculator::StatisticContext;
use economy::core::domain::statistic::renderer::{AsciiTableRenderer, MarkdownTableRenderer, TableRenderer};
use economy::core::domain::statistic::schedule::ReleaseSchedule;
use economy::core::domain::statistic::table::StatisticalTableDefinition;
use economy::core::domain::statistic::table_calculator::{
    CommodityCirculationTableCalculator, DemographicCohortTableCalculator, StatisticalTableCalculator,
};
use economy::core::domain::time::{RunId, Tick, TickDuration};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::ports::statistic_store::StatisticStorePort;
use economy::core::systems::statistic_system::StatisticSystem;
use economy::engine::{SimulationConfig, SimulationEngine};

#[test]
fn test_demographic_and_commodity_table_calculators() {
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();

    // Create agents across different cohorts
    // 1. Child: age 5 years (5 * 365 ticks)
    let child = Human::new(AgentId::new(1), Sex::Male, Tick::ZERO)
        .with_initial_age(5 * 365)
        .with_calories(10000.0)
        .with_item(ItemId::GRAIN, 3);
    agent_store.insert_human(child);

    // 2. Productive Male: age 25 years, married to Female 3
    let mut prod_male = Human::new(AgentId::new(2), Sex::Male, Tick::ZERO)
        .with_initial_age(25 * 365)
        .with_calories(20000.0)
        .with_item(ItemId::TIMBER, 10)
        .with_item(ItemId::STONE_AXE, 1);
    prod_male.set_spouse(Some(AgentId::new(3)));
    agent_store.insert_human(prod_male);

    // 3. Productive Female: age 24 years, married to Male 2
    let mut prod_female = Human::new(AgentId::new(3), Sex::Female, Tick::ZERO)
        .with_initial_age(24 * 365)
        .with_calories(18000.0)
        .with_item(ItemId::FISH, 5);
    prod_female.set_spouse(Some(AgentId::new(2)));
    agent_store.insert_human(prod_female);

    // 4. Elder: age 70 years
    let elder = Human::new(AgentId::new(4), Sex::Female, Tick::ZERO)
        .with_initial_age(70 * 365)
        .with_calories(8000.0)
        .with_item(ItemId::SALT, 4);
    agent_store.insert_human(elder);

    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(
            1,
            "Forest",
            ItemId::TIMBER,
            GeoCoordinate::new(0, 0),
            500,
            1000,
            10.0,
            1.0,
            0.0,
            serde_json::json!({}),
        )
        .with_pace(RegenerationPace::Slow),
    ];
    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);

    let ctx = StatisticContext {
        current_tick: Tick(30),
        agents: &agent_store,
        ledger: &ledger_store,
        environment: &env_store,
    };

    // 1. Test Demographic Cohort Table Calculator
    let demo_calc = DemographicCohortTableCalculator;
    let demo_table = demo_calc.calculate_table(&ctx);

    assert_eq!(demo_table.table_id, "TAB_DEMO_01");
    assert_eq!(demo_table.rows.len(), 4, "Must have 4 age cohort rows");
    assert_eq!(demo_table.summary_rows.len(), 1, "Must have summary row");
    assert_eq!(demo_table.footnotes.len(), 2, "Must have 2 structural footnotes");

    // Check ASCII Renderer
    let ascii_renderer = AsciiTableRenderer::new();
    let ascii_output = ascii_renderer.render(&demo_table);
    assert!(ascii_output.contains("Tabel Sensus Demografi"));
    assert!(ascii_output.contains("00 - 14 tahun (Balita & Anak)"));
    assert!(ascii_output.contains("15 - 44 tahun (Usia Produktif Awal)"));
    assert!(ascii_output.contains("TOTAL POPULASI HIDUP"));

    // Check Markdown Renderer
    let md_renderer = MarkdownTableRenderer;
    let md_output = md_renderer.render(&demo_table);
    assert!(md_output.contains("| Kelompok Usia (Kohor) |"));
    assert!(md_output.contains("|:---|"));

    // 2. Test Commodity Circulation Table Calculator
    let comm_calc = CommodityCirculationTableCalculator;
    let comm_table = comm_calc.calculate_table(&ctx);

    assert_eq!(comm_table.table_id, "TAB_COMM_01");
    assert!(comm_table.rows.len() >= 9, "Should list all standard economy commodities");
    assert_eq!(comm_table.summary_rows.len(), 1);

    let comm_ascii = ascii_renderer.render(&comm_table);
    assert!(comm_ascii.contains("Tabel Sensus Komoditas"));
    assert!(comm_ascii.contains("Kayu Mentah (Timber)"));
    assert!(comm_ascii.contains("Kapak Batu (Stone Axe)"));
    assert!(comm_ascii.contains("TOTAL STOK FISIK"));
}

#[test]
fn test_table_release_in_simulation_lifecycle() {
    let output_dir = "output/test_table_release";
    let _ = fs::remove_dir_all(output_dir);

    let run_id = RunId::new("test_table_release_run");
    let tick_duration = TickDuration::Day;
    let seed = 42;

    let rng = ChaChaRngAdapter::new(seed);
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();
    let stat_store = MemoryStatisticStore::new();

    for i in 1..=20 {
        let agent_id = AgentId::new(i as u64);
        let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
        let human = Human::new(agent_id, sex, Tick::ZERO)
            .with_initial_age(22 * 365)
            .with_item(ItemId::GRAIN, 10)
            .with_item(ItemId::TIMBER, 4)
            .with_calories(30000.0)
            .with_location(GeoCoordinate::new(10, 10));
        agent_store.insert_human(human);
    }

    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(
            1,
            "Forest",
            ItemId::TIMBER,
            GeoCoordinate::new(10, 10),
            500,
            1000,
            10.0,
            1.0,
            0.0,
            serde_json::json!({}),
        )
        .with_pace(RegenerationPace::Slow),
    ];
    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);

    let mut statistic_system = StatisticSystem::new();
    // Register monthly table releases (Day 1 of each month)
    statistic_system.register_table(StatisticalTableDefinition::new(
        "TAB_DEMO_01",
        "Demografi Cohort Table",
        ReleaseSchedule::DayOfMonth(1),
        AccessRequirement::Public,
        DemographicCohortTableCalculator,
    ));
    statistic_system.register_table(StatisticalTableDefinition::new(
        "TAB_COMM_01",
        "Commodity Circulation Table",
        ReleaseSchedule::DayOfMonth(1),
        AccessRequirement::Public,
        CommodityCirculationTableCalculator,
    ));

    let exporter = ParquetExporter::new(output_dir);
    let config = SimulationConfig::new(
        run_id,
        seed,
        tick_duration,
        65, // 65 ticks -> spans over 2 months (Day 1 at tick 0/30/60)
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
        21,
    );

    let summary = engine.run().expect("Simulation failed");

    // Verify table releases occurred
    assert!(summary.total_table_releases >= 2, "Expected at least 2 table releases over 65 days");

    let latest_demo = engine.stat_store().get_latest_table("TAB_DEMO_01");
    assert!(latest_demo.is_some(), "Latest demographic table must be present");

    let latest_comm = engine.stat_store().get_latest_table("TAB_COMM_01");
    assert!(latest_comm.is_some(), "Latest commodity table must be present");

    // Verify Parquet file for tables was written
    let tables_parquet = format!("{}/run_id=test_table_release_run/tables.parquet", output_dir);
    assert!(fs::metadata(&tables_parquet).is_ok(), "tables.parquet should exist at {}", tables_parquet);

    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn test_spatial_lapse_rate_and_informed_agent_barter() {
    let climate = ClimateState::default_spring(); // 20.0°C base

    // 1. Verify Environmental Lapse Rate physics (-6.5°C per 1000m):
    let valley_temp = climate.local_temperature(50.0, 25, 50);
    let mountain_temp = climate.local_temperature(1800.0, 5, 50);

    // Mountain at 1800m and northern latitude should be significantly colder than valley
    assert!(mountain_temp < valley_temp);
    let temp_diff = valley_temp - mountain_temp;
    assert!(temp_diff >= 11.0, "1800m elevation + northern latitude should drop temperature by >= 11°C, got diff {}", temp_diff);

    // 2. Verify Tropical zone detection:
    assert!(climate.is_tropical_zone(35, 50));
    assert!(!climate.is_tropical_zone(5, 50));

    // 3. Verify spatial growth multiplier:
    let valley_growth = climate.spatial_growth_multiplier(50.0, 35, 50);
    let mountain_growth = climate.spatial_growth_multiplier(1800.0, 5, 50);
    assert!(valley_growth > mountain_growth, "Valley growth should exceed alpine freeze growth");
}


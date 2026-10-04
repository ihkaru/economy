use clap::{Parser, ValueEnum};
use economy::adapters::persistence::ParquetExporter;
use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{
    MemoryAgentStore, MemoryEnvironmentStore, MemoryLedgerStore, MemoryStatisticStore,
};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::environment::climate::ClimateState;
use economy::core::domain::environment::resource::{RegenerationPace, ResourceNode};
use economy::core::domain::item::id::ItemId;
use economy::core::domain::statistic::access::AccessRequirement;
use economy::core::domain::statistic::calculator::{
    EmergentCurrencyCalculator, PopulationDemographyCalculator, ResourceScarcityCalculator,
    TradeVolumeCalculator, WealthDistributionCalculator,
};
use economy::core::domain::statistic::definition::StatisticDefinition;
use economy::core::domain::statistic::schedule::ReleaseSchedule;
use economy::core::domain::time::{RunId, Tick, TickDuration};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::systems::statistic_system::StatisticSystem;
use economy::engine::{SimulationConfig, SimulationEngine};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliTickDuration {
    Hour,
    Day,
    Month,
    Year,
}

impl From<CliTickDuration> for TickDuration {
    fn from(cli: CliTickDuration) -> Self {
        match cli {
            CliTickDuration::Hour => TickDuration::Hour,
            CliTickDuration::Day => TickDuration::Day,
            CliTickDuration::Month => TickDuration::Month,
            CliTickDuration::Year => TickDuration::Year,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Deterministic ABM Economic Simulation Engine")]
struct Cli {
    /// Explicit Run ID for simulation tracking
    #[arg(short, long)]
    run_id: Option<String>,

    /// Master seed for 100% deterministic pseudo-randomness
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Time duration represented by 1 tick
    #[arg(short, long, value_enum, default_value_t = CliTickDuration::Day)]
    duration: CliTickDuration,

    /// Total number of ticks to simulate
    #[arg(short, long, default_value_t = 365)]
    ticks: u64,

    /// Ticks per second pacing (omit for uncapped headless max speed)
    #[arg(long)]
    tps: Option<u64>,

    /// Initial population size
    #[arg(long, default_value_t = 50)]
    initial_agents: usize,

    /// Output directory for Apache Parquet persistence
    #[arg(short, long, default_value = "output")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let run_id = match cli.run_id {
        Some(id) => RunId::new(id),
        None => RunId::generate_random(),
    };

    let tick_duration: TickDuration = cli.duration.into();

    println!("=======================================================");
    println!("🏛️  DETERMINISTIC ABM ECONOMIC SIMULATOR (RUST)");
    println!("=======================================================");
    println!("Run ID           : {}", run_id.as_str());
    println!("Master Seed      : {}", cli.seed);
    println!("Tick Duration    : {:?}", tick_duration);
    println!("Total Ticks      : {}", cli.ticks);
    println!("Pacing (TPS)     : {:?}", cli.tps);
    println!("Initial Pop      : {}", cli.initial_agents);
    println!("Output Parquet   : {}", cli.output_dir);
    println!("-------------------------------------------------------");

    // 1. Setup Adapters (Dependency Injection Composition Root)
    let rng_adapter = ChaChaRngAdapter::new(cli.seed);
    let mut agent_store = MemoryAgentStore::new();
    let ledger_store = MemoryLedgerStore::new();
    let stat_store = MemoryStatisticStore::new();

    // Initialize human agents (balanced sex and realistic starting ages)
    let hours_per_tick = tick_duration.approximate_hours();
    let ticks_per_year = (24.0 * 365.0) / hours_per_tick;

    for i in 1..=cli.initial_agents {
        let agent_id = AgentId::new(i as u64);
        let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
        let initial_age_years = 20.0 + ((i % 11) as f64);
        let initial_age_ticks = (initial_age_years * ticks_per_year).round() as u64;

        // Place initial pioneer settlement near the central river valley (15, 25)
        let human = Human::new(agent_id, sex, Tick::ZERO)
            .with_initial_age(initial_age_ticks)
            .with_item(ItemId::GRAIN, 5) // Initial 5 units of grain food stock
            .with_item(ItemId::TIMBER, 2) // Basic crafting/firewood stock
            .with_location(economy::core::domain::spatial::GeoCoordinate::new(15, 25))
            .with_calories(25000.0); // 12-day initial caloric buffer

        agent_store.insert_human(human);
    }

    // Initialize environment and natural resource spawner nodes across biomes
    let climate = ClimateState::default_spring();
    let resource_nodes = vec![
        ResourceNode::new(
            1,
            "Ancient Oak Forest",
            ItemId::TIMBER,
            economy::core::domain::spatial::GeoCoordinate::new(15, 28),
            500,
            1000,
            10.0,
            2.0,
            0.0, // Non-edible building material
            serde_json::json!({"resource": "Timber", "biome": "Forest"}),
        )
        .with_pace(RegenerationPace::Slow),
        ResourceNode::new(
            2,
            "Silver Creek Fishery",
            ItemId::FISH,
            economy::core::domain::spatial::GeoCoordinate::new(15, 25),
            2500,
            10000,
            80.0,
            2.0,
            500.0, // Edible fresh fish
            serde_json::json!({"resource": "Fish", "biome": "River"}),
        )
        .with_pace(RegenerationPace::Medium),
        ResourceNode::new(
            3,
            "Sunlit Wheat Plains",
            ItemId::GRAIN,
            economy::core::domain::spatial::GeoCoordinate::new(16, 24),
            4000,
            20000,
            120.0,
            3.0,
            800.0, // Edible wild grain
            serde_json::json!({"resource": "Wheat", "biome": "Agricultural"}),
        )
        .with_pace(RegenerationPace::Medium),
        ResourceNode::new(
            4,
            "Wild Berry Woods",
            ItemId::BERRIES,
            economy::core::domain::spatial::GeoCoordinate::new(14, 26),
            1500,
            5000,
            60.0,
            2.0,
            300.0, // Edible berries
            serde_json::json!({"resource": "Berries", "biome": "Forest"}),
        )
        .with_pace(RegenerationPace::Fast),
        ResourceNode::new(
            5,
            "Volcanic Island Salt Mine",
            ItemId::SALT,
            economy::core::domain::spatial::GeoCoordinate::new(45, 25), // Offshore island across DeepOcean!
            800,
            2000,
            5.0,
            0.5,
            0.0,
            serde_json::json!({"resource": "RockSalt", "biome": "IslandVolcano", "maritime_required": true}),
        )
        .with_pace(RegenerationPace::Geological),
    ];
    let env_store = MemoryEnvironmentStore::new(climate, resource_nodes);

    // 2. Setup Statistical Publication Indicators & Data Openness Rules
    let mut statistic_system = StatisticSystem::new();

    // Indicator 1: Public Demography & Census (Daily)
    statistic_system.register(StatisticDefinition::new(
        "POP_DEMOGRAPHY",
        "Demografi & Sensus Penduduk",
        ReleaseSchedule::EveryDays(1),
        AccessRequirement::Public,
        PopulationDemographyCalculator,
    ));

    // Indicator 2: Public Natural Resource Reserves (Released on the 1st of every month)
    statistic_system.register(StatisticDefinition::new(
        "RESOURCE_RESERVES",
        "Neraca Cadangan Sumber Daya Alam",
        ReleaseSchedule::DayOfMonth(1),
        AccessRequirement::Public,
        ResourceScarcityCalculator,
    ));

    // Indicator 3: Tiered Asset Distribution Index (Weekly, requires asset inventory >= 10 units)
    statistic_system.register(StatisticDefinition::new(
        "WEALTH_DISTRIBUTION",
        "Indeks Distribusi Aset Fisik & Inventori",
        ReleaseSchedule::EveryDays(7),
        AccessRequirement::MinimumWealth(10),
        WealthDistributionCalculator,
    ));

    // Indicator 4: Public Quarterly Market Turnover (Every 3 months / Triwulanan)
    statistic_system.register(StatisticDefinition::new(
        "MARKET_TURNOVER",
        "Omset Pasar & Volume Rantai Pasok",
        ReleaseSchedule::EveryMonths(3),
        AccessRequirement::Public,
        TradeVolumeCalculator,
    ));

    // Indicator 5: Emergent Dominant Currency Tracker (Monthly on the 1st)
    statistic_system.register(StatisticDefinition::new(
        "EMERGENT_CURRENCY",
        "Indikator Komoditas Uang Dominan (Velocity of Money)",
        ReleaseSchedule::DayOfMonth(1),
        AccessRequirement::Public,
        EmergentCurrencyCalculator,
    ));

    let exporter = ParquetExporter::new(&cli.output_dir);

    // 3. Configure Simulation
    let config = SimulationConfig::new(
        run_id,
        cli.seed,
        tick_duration,
        cli.ticks,
        cli.tps,
        &cli.output_dir,
    );

    // 4. Instantiate Engine via Constructor Injection
    let mut engine = SimulationEngine::new(
        config,
        agent_store,
        ledger_store,
        env_store,
        stat_store,
        rng_adapter,
        exporter,
        statistic_system,
        (cli.initial_agents + 1) as u64,
    );

    // 5. Run Simulation
    println!("🚀 Starting simulation execution...");
    let summary = engine.run()?;

    println!("-------------------------------------------------------");
    println!("✅ SIMULATION COMPLETED SUCCESSFULLY");
    println!("-------------------------------------------------------");
    println!("Elapsed Sim Time : {:.2} years ({} ticks)", summary.elapsed_simulation_years, summary.total_ticks_completed);
    println!("Wall Clock Time  : {:.4} seconds", summary.wall_clock_duration_secs);
    println!("Simulation Speed : {:.1} TPS (Ticks/sec)", summary.average_tps);
    println!("Agent Population : {} living / {} total (including deceased/born)", summary.living_agents, summary.total_agents);
    println!("Ledger Records   : {} transactions in Ultimate Ledger", summary.total_ledger_transactions);
    println!("Stat Publications: {} official indicator releases", summary.total_statistical_releases);
    println!("Parquet Exporter : Output saved to {}", summary.export_summary.output_directory);
    println!("=======================================================");

    Ok(())
}

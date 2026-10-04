use std::thread;
use std::time::{Duration, Instant};

use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::time::SimulationClock;
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::export_port::{ExportPort, ExportSummary};
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::rng_port::RngPort;
use crate::core::ports::statistic_store::StatisticStorePort;
use crate::core::systems::environment_system::EnvironmentSystem;
use crate::core::systems::exchange_system::ExchangeSystem;
use crate::core::systems::lifecycle_system::LifecycleSystem;
use crate::core::systems::statistic_system::StatisticSystem;
use crate::engine::config::SimulationConfig;

#[derive(Debug, Clone)]
pub struct SimulationSummary {
    pub run_id: String,
    pub total_ticks_completed: u64,
    pub elapsed_simulation_years: f64,
    pub wall_clock_duration_secs: f64,
    pub average_tps: f64,
    pub living_agents: usize,
    pub total_agents: usize,
    pub total_ledger_transactions: usize,
    pub total_statistical_releases: usize,
    pub total_table_releases: usize,
    pub export_summary: ExportSummary,
}

pub struct SimulationEngine<A, L, E, S, R, P>
where
    A: AgentStorePort,
    L: LedgerStorePort,
    E: EnvironmentStorePort,
    S: StatisticStorePort,
    R: RngPort,
    P: ExportPort,
{
    config: SimulationConfig,
    agent_store: A,
    ledger_store: L,
    env_store: E,
    stat_store: S,
    rng: R,
    exporter: P,
    lifecycle_system: LifecycleSystem,
    metabolism_system: crate::core::systems::metabolism_system::MetabolismSystem,
    environment_system: EnvironmentSystem,
    exchange_system: ExchangeSystem,
    statistic_system: StatisticSystem,
    clock: SimulationClock,
}

impl<A, L, E, S, R, P> SimulationEngine<A, L, E, S, R, P>
where
    A: AgentStorePort,
    L: LedgerStorePort,
    E: EnvironmentStorePort,
    S: StatisticStorePort,
    R: RngPort,
    P: ExportPort,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: SimulationConfig,
        agent_store: A,
        ledger_store: L,
        env_store: E,
        stat_store: S,
        rng: R,
        exporter: P,
        statistic_system: StatisticSystem,
        initial_next_agent_id: u64,
    ) -> Self {
        let clock = SimulationClock::new(config.tick_duration);
        Self {
            config,
            agent_store,
            ledger_store,
            env_store,
            stat_store,
            rng,
            exporter,
            lifecycle_system: LifecycleSystem::new(initial_next_agent_id),
            metabolism_system: crate::core::systems::metabolism_system::MetabolismSystem::new(2000.0),
            environment_system: EnvironmentSystem::new(),
            exchange_system: ExchangeSystem::new(1),
            statistic_system,
            clock,
        }
    }

    pub fn register_statistic(&mut self, definition: StatisticDefinition) {
        self.statistic_system.register(definition);
    }

    pub fn register_table_statistic(&mut self, definition: crate::core::domain::statistic::table::StatisticalTableDefinition) {
        self.statistic_system.register_table(definition);
    }

    pub fn stat_store(&self) -> &S {
        &self.stat_store
    }

    pub fn ledger_store(&self) -> &L {
        &self.ledger_store
    }

    pub fn agent_store(&self) -> &A {
        &self.agent_store
    }

    pub fn env_store(&self) -> &E {
        &self.env_store
    }

    /// Run the simulation to completion deterministically with optional rate-limiting
    pub fn run(&mut self) -> Result<SimulationSummary, String> {
        let wall_start = Instant::now();
        let target_frame_duration = self.config.ticks_per_second.and_then(|tps| {
            1_000_000_000_u64.checked_div(tps).map(Duration::from_nanos)
        });

        for _ in 0..self.config.total_ticks {
            let tick_start = Instant::now();
            let current_tick = self.clock.advance_tick();

            // 1. Environment System (Seasons, Weather, Wind, Natural Resource Regeneration/Decay)
            self.environment_system.step(
                current_tick,
                self.config.tick_duration,
                &mut self.env_store,
                &mut self.rng,
            );

            // 2. Metabolism System (Daily Caloric Burn, Survival Foraging, Starvation Hazard)
            self.metabolism_system.step(
                current_tick,
                &mut self.agent_store,
                &mut self.env_store,
                &mut self.rng,
            );

            // 3. Lifecycle System (Aging, Marriage, Birth, Natural Mortality)
            self.lifecycle_system.step(
                current_tick,
                self.config.tick_duration,
                &mut self.agent_store,
                &mut self.rng,
            );

            // 4. Exchange System (Resource Harvesting, Supply Chain Trading, Boat Crafting, Ultimate Ledger)
            self.exchange_system.step(
                &self.config.run_id,
                current_tick,
                &mut self.agent_store,
                &mut self.env_store,
                &mut self.ledger_store,
                &self.stat_store,
                &mut self.rng,
            );

            // 5. Statistic System (Scheduled Indicator Calculations & Data Transparency Publications)
            self.statistic_system.step(
                current_tick,
                &self.clock,
                &self.agent_store,
                &self.ledger_store,
                &self.env_store,
                &mut self.stat_store,
            );

            // Rate-limiting pacing if requested
            if let Some(target_dur) = target_frame_duration {
                let elapsed = tick_start.elapsed();
                if elapsed < target_dur {
                    thread::sleep(target_dur - elapsed);
                }
            }
        }

        let wall_duration = wall_start.elapsed();
        let wall_duration_secs = wall_duration.as_secs_f64();
        let average_tps = if wall_duration_secs > 0.0 {
            self.config.total_ticks as f64 / wall_duration_secs
        } else {
            0.0
        };

        // Export in-memory state, ledger, and statistical publications to Apache Parquet
        let all_humans = self.agent_store.get_all_humans();
        let export_summary = self.exporter.export_all(
            &self.config.run_id,
            self.ledger_store.all_entries(),
            &all_humans,
            self.env_store.climate(),
            self.env_store.nodes(),
            self.stat_store.all_releases(),
            self.stat_store.all_table_releases(),
        )?;

        Ok(SimulationSummary {
            run_id: self.config.run_id.as_str().to_string(),
            total_ticks_completed: self.config.total_ticks,
            elapsed_simulation_years: self.clock.elapsed_years(),
            wall_clock_duration_secs: wall_duration_secs,
            average_tps,
            living_agents: self.agent_store.count_alive(),
            total_agents: all_humans.len(),
            total_ledger_transactions: self.ledger_store.total_records(),
            total_statistical_releases: self.stat_store.total_releases(),
            total_table_releases: self.stat_store.total_table_releases(),
            export_summary,
        })
    }
}

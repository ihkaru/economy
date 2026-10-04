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

        let heartbeat_interval = (self.config.total_ticks / 20).max(365).min(3650);
        let mut last_heartbeat_time = Instant::now();
        let mut last_heartbeat_tick = 0_u64;

        let mut time_env = Duration::ZERO;
        let mut time_metabolism = Duration::ZERO;
        let mut time_lifecycle = Duration::ZERO;
        let mut time_exchange = Duration::ZERO;
        let mut time_statistic = Duration::ZERO;

        for tick_idx in 0..self.config.total_ticks {
            let tick_num = tick_idx + 1;
            let tick_start = Instant::now();
            let current_tick = self.clock.advance_tick();

            // 1. Environment System (Seasons, Weather, Wind, Natural Resource Regeneration/Decay)
            let t0 = Instant::now();
            self.environment_system.step(
                current_tick,
                self.config.tick_duration,
                &mut self.env_store,
                &mut self.rng,
            );
            time_env += t0.elapsed();

            // 2. Metabolism System (Daily Caloric Burn, Survival Foraging, Starvation Hazard)
            let t1 = Instant::now();
            self.metabolism_system.step(
                current_tick,
                &mut self.agent_store,
                &mut self.env_store,
                &mut self.rng,
            );
            time_metabolism += t1.elapsed();

            // 3. Lifecycle System (Aging, Marriage, Birth, Natural Mortality)
            let t2 = Instant::now();
            self.lifecycle_system.step(
                current_tick,
                self.config.tick_duration,
                &mut self.agent_store,
                &mut self.rng,
            );
            time_lifecycle += t2.elapsed();

            // 4. Exchange System (Resource Harvesting, Supply Chain Trading, Boat Crafting, Ultimate Ledger)
            let t3 = Instant::now();
            self.exchange_system.step(
                &self.config.run_id,
                current_tick,
                &mut self.agent_store,
                &mut self.env_store,
                &mut self.ledger_store,
                &self.stat_store,
                &mut self.rng,
            );
            time_exchange += t3.elapsed();

            // 5. Statistic System (Scheduled Indicator Calculations & Data Transparency Publications)
            let t4 = Instant::now();
            self.statistic_system.step(
                current_tick,
                &self.clock,
                &self.agent_store,
                &self.ledger_store,
                &self.env_store,
                &mut self.stat_store,
            );
            time_statistic += t4.elapsed();

            // Rate-limiting pacing if requested
            if let Some(target_dur) = target_frame_duration {
                let elapsed = tick_start.elapsed();
                if elapsed < target_dur {
                    thread::sleep(target_dur - elapsed);
                }
            }

            // Periodic Live Observability Heartbeat
            if tick_num % heartbeat_interval == 0 || tick_num == self.config.total_ticks {
                let pct = (tick_num as f64 / self.config.total_ticks as f64) * 100.0;
                let current_year = (current_tick.0 as f64) / 365.0;
                let living = self.agent_store.count_alive();
                let total_pop = self.agent_store.count_total();
                let delta_ticks = tick_num.saturating_sub(last_heartbeat_tick);
                let delta_secs = last_heartbeat_time.elapsed().as_secs_f64();
                let inst_tps = if delta_secs > 0.0 { delta_ticks as f64 / delta_secs } else { 0.0 };
                let remaining_ticks = self.config.total_ticks.saturating_sub(tick_num);
                let eta_secs = if inst_tps > 0.0 { remaining_ticks as f64 / inst_tps } else { 0.0 };

                println!(
                    "⏱️  [Year {:>4.0} | Tick {:>6} ({:>5.1}%)] Living: {:>3} | Total: {:>4} | Speed: {:>6.0} TPS | ETA: {:>4.1}s",
                    current_year, current_tick.0, pct, living, total_pop, inst_tps, eta_secs
                );
                last_heartbeat_tick = tick_num;
                last_heartbeat_time = Instant::now();
            }
        }

        let wall_duration = wall_start.elapsed();
        let wall_duration_secs = wall_duration.as_secs_f64();
        let average_tps = if wall_duration_secs > 0.0 {
            self.config.total_ticks as f64 / wall_duration_secs
        } else {
            0.0
        };

        // Engine Micro-Profiling Performance Breakdown
        let total_sys_time = (time_env + time_metabolism + time_lifecycle + time_exchange + time_statistic).as_secs_f64();
        let pct_of = |d: Duration| if total_sys_time > 0.0 { (d.as_secs_f64() / total_sys_time) * 100.0 } else { 0.0 };
        let avg_us = |d: Duration| (d.as_secs_f64() * 1_000_000.0) / self.config.total_ticks.max(1) as f64;

        println!("\n======================================================================");
        println!("⏱️  ENGINE SUBSYSTEM PROFILING & OBSERVABILITY BREAKDOWN");
        println!("======================================================================");
        println!("┌────────────────────────────┬──────────────┬────────────┬───────────────────┐");
        println!("│ Subsystem Component        │ Total Time   │ Share (%)  │ Avg Latency/Tick  │");
        println!("├────────────────────────────┼──────────────┼────────────┼───────────────────┤");
        println!("│ ExchangeSystem             │ {:>10.2} s │ {:>9.1}% │ {:>13.3} µs │", time_exchange.as_secs_f64(), pct_of(time_exchange), avg_us(time_exchange));
        println!("│ MetabolismSystem           │ {:>10.2} s │ {:>9.1}% │ {:>13.3} µs │", time_metabolism.as_secs_f64(), pct_of(time_metabolism), avg_us(time_metabolism));
        println!("│ StatisticSystem            │ {:>10.2} s │ {:>9.1}% │ {:>13.3} µs │", time_statistic.as_secs_f64(), pct_of(time_statistic), avg_us(time_statistic));
        println!("│ LifecycleSystem            │ {:>10.2} s │ {:>9.1}% │ {:>13.3} µs │", time_lifecycle.as_secs_f64(), pct_of(time_lifecycle), avg_us(time_lifecycle));
        println!("│ EnvironmentSystem          │ {:>10.2} s │ {:>9.1}% │ {:>13.3} µs │", time_env.as_secs_f64(), pct_of(time_env), avg_us(time_env));
        println!("├────────────────────────────┼──────────────┼────────────┼───────────────────┤");
        println!("│ Pure Subsystem Computation │ {:>10.2} s │    100.0%  │ {:>13.3} µs │", total_sys_time, (total_sys_time * 1_000_000.0) / self.config.total_ticks.max(1) as f64);
        println!("│ Total Wall-Clock Execution │ {:>10.2} s │         -  │ {:>13.3} µs │", wall_duration_secs, (wall_duration_secs * 1_000_000.0) / self.config.total_ticks.max(1) as f64);
        println!("└────────────────────────────┴──────────────┴────────────┴───────────────────┘");

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

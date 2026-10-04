use crate::core::domain::statistic::calculator::StatisticContext;
use crate::core::domain::statistic::definition::StatisticDefinition;
use crate::core::domain::statistic::record::StatisticReleaseRecord;
use crate::core::domain::time::{SimulationClock, Tick};
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;
use crate::core::ports::statistic_store::StatisticStorePort;

pub struct StatisticSystem {
    definitions: Vec<StatisticDefinition>,
    next_release_id: u64,
}

impl StatisticSystem {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
            next_release_id: 1,
        }
    }

    pub fn register(&mut self, definition: StatisticDefinition) {
        self.definitions.push(definition);
    }

    pub fn definitions(&self) -> &[StatisticDefinition] {
        &self.definitions
    }

    /// Step the statistical publication engine
    pub fn step(
        &mut self,
        current_tick: Tick,
        clock: &SimulationClock,
        agent_store: &dyn AgentStorePort,
        ledger_store: &dyn LedgerStorePort,
        env_store: &dyn EnvironmentStorePort,
        stat_store: &mut dyn StatisticStorePort,
    ) {
        let ctx = StatisticContext {
            current_tick,
            agents: agent_store,
            ledger: ledger_store,
            environment: env_store,
        };

        for def in &self.definitions {
            if def.schedule.is_due(current_tick, clock) {
                let computed = def.calculator.calculate(&ctx);

                let record = StatisticReleaseRecord {
                    release_id: self.next_release_id,
                    statistic_id: def.id.clone(),
                    name: def.name.clone(),
                    release_tick: current_tick,
                    schedule_desc: def.schedule.description(),
                    primary_value: computed.primary_scalar,
                    payload: computed.payload,
                    access_requirement: serde_json::to_value(&def.access).unwrap_or_default(),
                };
                self.next_release_id += 1;

                let _ = stat_store.record_release(record);
            }
        }
    }
}

impl Default for StatisticSystem {
    fn default() -> Self {
        Self::new()
    }
}

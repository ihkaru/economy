use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::core::domain::item::id::ItemId;

/// Subjective internal economic statistics and lagged market memory attached to an individual agent.
/// Emulates bounded rationality and information friction (Hayek 1945, Simon 1957).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersonalStats {
    /// Total successful bilateral transactions executed by this agent
    pub trade_count: u64,
    /// Cumulative subjective economic surplus (utility) acquired from trades
    pub cumulative_surplus: f64,
    /// Tick when the agent last received fresh market information (from bulletin or peer gossip)
    pub last_observation_tick: u64,
    /// Perceived subjective scarcity multiplier per item (1.0 = normal, >1.0 = scarce, <1.0 = abundant)
    pub scarcity_beliefs: BTreeMap<u64, f64>,
}

impl Default for AgentPersonalStats {
    fn default() -> Self {
        Self {
            trade_count: 0,
            cumulative_surplus: 0.0,
            last_observation_tick: 0,
            scarcity_beliefs: BTreeMap::new(),
        }
    }
}

impl AgentPersonalStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates the agent's subjective scarcity valuation for an item at current_tick.
    /// Accounts for information lag: beliefs decay toward neutral 1.0 as memory gets stale without updates.
    pub fn get_scarcity_multiplier(&self, item_id: ItemId, current_tick: u64) -> f64 {
        let base_belief = *self.scarcity_beliefs.get(&item_id.as_u64()).unwrap_or(&1.0);
        let lag = current_tick.saturating_sub(self.last_observation_tick);

        // Information decay: if information is older than 60 days, confidence decays toward neutral 1.0
        if lag > 60 {
            let decay_factor = (-(lag as f64 - 60.0) / 180.0).exp();
            1.0 + (base_belief - 1.0) * decay_factor
        } else {
            base_belief
        }
    }

    /// Observes market signals with an enforced time lag (no real-time omniscience).
    /// Uses adaptive expectations (EWMA: 70% memory retention, 30% new lagged signal).
    pub fn observe_market_with_lag(&mut self, item_id: ItemId, signal: f64, current_tick: u64, lag_ticks: u64) {
        let current_belief = *self.scarcity_beliefs.get(&item_id.as_u64()).unwrap_or(&1.0);
        let updated_belief = (0.70 * current_belief + 0.30 * signal).clamp(0.65, 1.65);
        self.scarcity_beliefs.insert(item_id.as_u64(), updated_belief);
        self.last_observation_tick = current_tick.saturating_sub(lag_ticks);
    }

    /// Spontaneous bilateral word-of-mouth diffusion: peer-to-peer transmission of market news.
    /// The agent with fresher information (lower lag) transmits price signals to the other with transmission friction.
    pub fn diffuse_information(&mut self, other: &mut AgentPersonalStats, current_tick: u64) {
        let my_lag = current_tick.saturating_sub(self.last_observation_tick);
        let other_lag = current_tick.saturating_sub(other.last_observation_tick);

        if my_lag + 10 < other_lag {
            // Self shares fresher beliefs with other (with 10-day transmission delay)
            for (&item_id, &belief) in &self.scarcity_beliefs {
                let other_cur = *other.scarcity_beliefs.get(&item_id).unwrap_or(&1.0);
                other.scarcity_beliefs.insert(item_id, 0.60 * other_cur + 0.40 * belief);
            }
            other.last_observation_tick = self.last_observation_tick.saturating_sub(10);
        } else if other_lag + 10 < my_lag {
            // Other shares fresher beliefs with self
            for (&item_id, &belief) in &other.scarcity_beliefs {
                let my_cur = *self.scarcity_beliefs.get(&item_id).unwrap_or(&1.0);
                self.scarcity_beliefs.insert(item_id, 0.60 * my_cur + 0.40 * belief);
            }
            self.last_observation_tick = other.last_observation_tick.saturating_sub(10);
        }
    }

    /// Records a completed transaction and updates experiential capital
    pub fn record_trade(&mut self, surplus: f64) {
        self.trade_count += 1;
        self.cumulative_surplus += surplus.max(0.0);
    }
}

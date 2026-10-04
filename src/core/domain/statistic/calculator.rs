use serde::{Deserialize, Serialize};
use crate::core::domain::agent::human::Human;
use crate::core::domain::agent::traits::HasLifecycle;
use crate::core::domain::time::Tick;
use crate::core::ports::agent_store::AgentStorePort;
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::ledger_store::LedgerStorePort;

pub struct StatisticContext<'a> {
    pub current_tick: Tick,
    pub agents: &'a dyn AgentStorePort,
    pub ledger: &'a dyn LedgerStorePort,
    pub environment: &'a dyn EnvironmentStorePort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticValue {
    /// Optional primary scalar value for fast aggregation queries
    pub primary_scalar: Option<f64>,
    /// Full multidimensional payload
    pub payload: serde_json::Value,
}

impl StatisticValue {
    pub fn scalar_and_payload(scalar: f64, payload: serde_json::Value) -> Self {
        Self {
            primary_scalar: Some(scalar),
            payload,
        }
    }

    pub fn payload_only(payload: serde_json::Value) -> Self {
        Self {
            primary_scalar: None,
            payload,
        }
    }
}

/// Interface-First strategy contract for economic and demographic statistical calculators
pub trait StatisticCalculator: Send + Sync {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue;
}

// ============================================================================
// Built-in Standard Economic & Demographic Calculators
// ============================================================================

/// Calculates living population, sex breakdown, and vital statistics
pub struct PopulationDemographyCalculator;

impl StatisticCalculator for PopulationDemographyCalculator {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue {
        let all_humans = ctx.agents.get_all_humans();
        let total_count = all_humans.len();
        let living_humans: Vec<&Human> = all_humans.iter().filter(|h| h.is_alive()).collect();
        let living_count = living_humans.len();
        let deceased_count = total_count.saturating_sub(living_count);

        let male_count = living_humans.iter().filter(|h| h.sex == crate::core::domain::agent::human::Sex::Male).count();
        let female_count = living_humans.iter().filter(|h| h.sex == crate::core::domain::agent::human::Sex::Female).count();
        let married_count = living_humans.iter().filter(|h| h.spouse_id.is_some()).count();

        StatisticValue::scalar_and_payload(
            living_count as f64,
            serde_json::json!({
                "living_population": living_count,
                "total_historical_population": total_count,
                "deceased_count": deceased_count,
                "males": male_count,
                "females": female_count,
                "married_agents": married_count,
            }),
        )
    }
}

/// Calculates physical asset distribution, inventory accumulation, and living agent count
pub struct WealthDistributionCalculator;

impl StatisticCalculator for WealthDistributionCalculator {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue {
        let all_humans = ctx.agents.get_all_humans();
        let living: Vec<&Human> = all_humans.iter().filter(|h| h.is_alive()).collect();
        let count = living.len();

        let total_assets: u64 = living
            .iter()
            .map(|h| h.inventory.values().sum::<u32>() as u64)
            .sum();
        let avg_assets = if count > 0 {
            total_assets as f64 / count as f64
        } else {
            0.0
        };

        let min_assets = living
            .iter()
            .map(|h| h.inventory.values().sum::<u32>())
            .min()
            .unwrap_or(0);
        let max_assets = living
            .iter()
            .map(|h| h.inventory.values().sum::<u32>())
            .max()
            .unwrap_or(0);

        StatisticValue::scalar_and_payload(
            avg_assets,
            serde_json::json!({
                "sample_size": count,
                "total_physical_assets": total_assets,
                "average_inventory_units": avg_assets,
                "min_inventory_units": min_assets,
                "max_inventory_units": max_assets,
            }),
        )
    }
}

/// Tracks the spontaneous emergence of commodity money by measuring trade velocity across items
pub struct EmergentCurrencyCalculator;

impl StatisticCalculator for EmergentCurrencyCalculator {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue {
        let all_entries = ctx.ledger.all_entries();
        let mut trade_counts: std::collections::BTreeMap<u64, u64> = std::collections::BTreeMap::new();

        for entry in all_entries {
            // Count bilateral trades (inter-agent market exchanges where both parties exchange goods/services/knowledge)
            if !entry.items_from_a.is_empty() && !entry.items_from_b.is_empty() {
                for it1 in &entry.items_from_a {
                    *trade_counts.entry(it1.item_id.0).or_insert(0) += 1;
                }
                for it2 in &entry.items_from_b {
                    *trade_counts.entry(it2.item_id.0).or_insert(0) += 1;
                }
            }
        }

        let mut dominant_item = 0;
        let mut max_trades = 0;
        for (item_id, count) in &trade_counts {
            if *count > max_trades {
                max_trades = *count;
                dominant_item = *item_id;
            }
        }

        let dominant_name = match dominant_item {
            101 => "Timber",
            102 => "Fish",
            103 => "Grain",
            104 => "Berries",
            105 => "Salt",
            106 => "Raft",
            107 => "Shells",
            _ => "None / Pure Direct Barter",
        };

        StatisticValue::scalar_and_payload(
            dominant_item as f64,
            serde_json::json!({
                "dominant_currency_item_id": dominant_item,
                "dominant_currency_name": dominant_name,
                "velocity_trade_count": max_trades,
                "item_trade_frequencies": trade_counts,
            }),
        )
    }
}

/// Calculates natural resource stocks, capacity utilization, and environmental scarcity ratio
pub struct ResourceScarcityCalculator;

impl StatisticCalculator for ResourceScarcityCalculator {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue {
        let nodes = ctx.environment.nodes();
        let total_stock: u32 = nodes.iter().map(|n| n.current_stock).sum();
        let total_capacity: u32 = nodes.iter().map(|n| n.max_stock).sum();

        let scarcity_ratio = if total_capacity > 0 {
            total_stock as f64 / total_capacity as f64
        } else {
            0.0
        };

        let climate = ctx.environment.climate();

        StatisticValue::scalar_and_payload(
            scarcity_ratio,
            serde_json::json!({
                "total_resource_stock": total_stock,
                "total_max_capacity": total_capacity,
                "scarcity_ratio": scarcity_ratio,
                "current_season": format!("{:?}", climate.season),
                "weather": format!("{:?}", climate.weather),
                "temperature_celsius": climate.temperature_celsius,
            }),
        )
    }
}

/// Calculates total market trading volume and activity recorded in the Ultimate Ledger
pub struct TradeVolumeCalculator;

impl StatisticCalculator for TradeVolumeCalculator {
    fn calculate(&self, ctx: &StatisticContext) -> StatisticValue {
        let total_transactions = ctx.ledger.total_records();
        let all_entries = ctx.ledger.all_entries();

        let market_trades = all_entries
            .iter()
            .filter(|e| !e.items_from_a.is_empty() && !e.items_from_b.is_empty())
            .count();

        let harvest_trades = total_transactions.saturating_sub(market_trades);

        StatisticValue::scalar_and_payload(
            total_transactions as f64,
            serde_json::json!({
                "total_ledger_entries": total_transactions,
                "inter_agent_trades": market_trades,
                "natural_resource_harvests": harvest_trades,
            }),
        )
    }
}

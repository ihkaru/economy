use serde::{Deserialize, Serialize};
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
        let total_count = ctx.agents.count_total();
        let mut living_count = 0;
        let mut male_count = 0;
        let mut female_count = 0;
        let mut married_count = 0;

        for h in ctx.agents.iter_living_humans() {
            living_count += 1;
            match h.sex {
                crate::core::domain::agent::human::Sex::Male => male_count += 1,
                crate::core::domain::agent::human::Sex::Female => female_count += 1,
            }
            if h.spouse_id.is_some() {
                married_count += 1;
            }
        }
        let deceased_count = total_count.saturating_sub(living_count);

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
        let mut count = 0;
        let mut total_assets: u64 = 0;
        let mut min_assets: u32 = u32::MAX;
        let mut max_assets: u32 = 0;

        for h in ctx.agents.iter_living_humans() {
            count += 1;
            let inv_sum: u32 = h.inventory.values().sum();
            total_assets += inv_sum as u64;
            if inv_sum < min_assets {
                min_assets = inv_sum;
            }
            if inv_sum > max_assets {
                max_assets = inv_sum;
            }
        }
        if count == 0 {
            min_assets = 0;
        }
        let avg_assets = if count > 0 {
            total_assets as f64 / count as f64
        } else {
            0.0
        };

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
        let trade_counts = ctx.ledger.bilateral_trade_item_counts();

        let mut dominant_item = 0;
        let mut max_trades = 0;
        let mut frequencies: std::collections::BTreeMap<u64, u64> = std::collections::BTreeMap::new();

        for (item_id, count) in trade_counts {
            frequencies.insert(item_id.0, *count);
            if *count > max_trades {
                max_trades = *count;
                dominant_item = item_id.0;
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
                "item_trade_frequencies": frequencies,
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
        let market_trades = ctx.ledger.bilateral_market_trades_count();
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

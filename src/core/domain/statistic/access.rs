use serde::{Deserialize, Serialize};
use crate::core::domain::agent::human::Human;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessRequirement {
    /// Full data openness: Available to everyone in the economy
    Public,
    /// Paid / Wealth-gated tier: Requires a minimum asset threshold (e.g. inventory accumulation)
    MinimumWealth(u64),
    /// Role-based access: Requires specific institution/role (e.g. "Government", "CentralBank", "LicensedTrader")
    RequiredRole(String),
    /// Arbitrary attribute condition evaluated against agent's attributes JSON
    AttributeMatch {
        key: String,
        expected_value: serde_json::Value,
    },
    /// Composite condition (AND logic): All conditions must be satisfied
    AllOf(Vec<AccessRequirement>),
    /// Composite condition (OR logic): At least one condition must be satisfied
    AnyOf(Vec<AccessRequirement>),
}

impl AccessRequirement {
    /// Evaluates whether an agent has the right to inspect this statistical data
    pub fn is_eligible(&self, agent: &Human) -> bool {
        match self {
            Self::Public => true,
            Self::MinimumWealth(min_wealth) => {
                let total_assets: u64 = agent.inventory.values().sum::<u32>() as u64;
                total_assets >= *min_wealth
            }
            Self::RequiredRole(role) => agent
                .attributes
                .get("role")
                .and_then(|r| r.as_str())
                .map(|r| r == role)
                .unwrap_or(false),
            Self::AttributeMatch { key, expected_value } => {
                agent.attributes.get(key) == Some(expected_value)
            }
            Self::AllOf(reqs) => reqs.iter().all(|r| r.is_eligible(agent)),
            Self::AnyOf(reqs) => reqs.iter().any(|r| r.is_eligible(agent)),
        }
    }
}

pub mod market_intelligence;
pub mod trade;

pub use market_intelligence::{agent_has_market_access, derive_market_scarcity_multiplier, evaluate_marginal_utility};
pub use trade::perform_trade_and_services;

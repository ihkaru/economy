pub mod id;
pub mod traits;
pub mod human;
pub mod stats;

pub use id::AgentId;
pub use traits::{Identifiable, HasLifecycle, SocialActor, EconomicActor};
pub use human::{Human, Sex, VitalStatus};
pub use stats::AgentPersonalStats;

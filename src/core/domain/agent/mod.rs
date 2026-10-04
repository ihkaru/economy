pub mod id;
pub mod traits;
pub mod human;

pub use id::AgentId;
pub use traits::{Identifiable, HasLifecycle, SocialActor, EconomicActor};
pub use human::{Human, Sex, VitalStatus};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    DeepOcean,
    ShallowWater,
    Plains,
    Forest,
    Mountain,
    River,
}

impl TerrainType {
    /// Base movement friction multiplier (higher = slower / more caloric expenditure)
    pub fn movement_friction(&self) -> f64 {
        match self {
            Self::Plains => 1.0,
            Self::River => 1.5,
            Self::Forest => 2.0,
            Self::ShallowWater => 4.0,
            Self::Mountain => 5.0,
            Self::DeepOcean => f64::INFINITY, // Impassable on foot
        }
    }

    /// Whether this terrain allows human traversal on foot without maritime vessels
    pub fn is_passable_on_foot(&self) -> bool {
        !matches!(self, Self::DeepOcean)
    }

    /// Whether water vessel (raft/boat) is required to enter
    pub fn requires_boat(&self) -> bool {
        matches!(self, Self::DeepOcean)
    }

    /// Whether this terrain is a water body
    pub fn is_water(&self) -> bool {
        matches!(self, Self::River | Self::ShallowWater | Self::DeepOcean)
    }

    /// Whether this terrain provides fresh water for drinking/hydration
    pub fn is_fresh_water(&self) -> bool {
        matches!(self, Self::River | Self::ShallowWater)
    }

    /// Agricultural / Foraging fertility multiplier
    pub fn soil_fertility(&self) -> f64 {
        match self {
            Self::Plains => 1.5,
            Self::River => 1.8,
            Self::Forest => 1.2,
            Self::Mountain => 0.3,
            Self::ShallowWater => 0.5,
            Self::DeepOcean => 0.0,
        }
    }
}

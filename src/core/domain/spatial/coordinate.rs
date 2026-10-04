use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GeoCoordinate {
    pub x: u32,
    pub y: u32,
}

impl GeoCoordinate {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Manhattan distance between two coordinates
    pub fn manhattan_distance(&self, other: &GeoCoordinate) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Euclidean distance squared (avoids float square root in fast checks)
    pub fn distance_squared(&self, other: &GeoCoordinate) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        dx * dx + dy * dy
    }

    pub fn euclidean_distance(&self, other: &GeoCoordinate) -> f64 {
        (self.distance_squared(other) as f64).sqrt()
    }
}

impl std::fmt::Display for GeoCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

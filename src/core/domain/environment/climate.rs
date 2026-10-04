use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Multiplier for natural resource regeneration
    pub fn growth_multiplier(&self) -> f64 {
        match self {
            Season::Spring => 1.5,
            Season::Summer => 1.2,
            Season::Autumn => 0.8,
            Season::Winter => 0.2,
        }
    }

    /// Multiplier for decay rate
    pub fn decay_multiplier(&self) -> f64 {
        match self {
            Season::Spring => 0.8,
            Season::Summer => 1.1,
            Season::Autumn => 1.3,
            Season::Winter => 1.8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    Rain,
    Storm,
    Drought,
}

impl WeatherCondition {
    pub fn growth_impact(&self) -> f64 {
        match self {
            WeatherCondition::Clear => 1.0,
            WeatherCondition::Rain => 1.3,
            WeatherCondition::Storm => 0.4,
            WeatherCondition::Drought => 0.2,
        }
    }

    pub fn decay_impact(&self) -> f64 {
        match self {
            WeatherCondition::Clear => 1.0,
            WeatherCondition::Rain => 1.1,
            WeatherCondition::Storm => 2.0,
            WeatherCondition::Drought => 1.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wind {
    pub speed_kmh: f64,
    pub direction_degrees: f64,
}

impl Wind {
    pub fn new(speed_kmh: f64, direction_degrees: f64) -> Self {
        Self {
            speed_kmh,
            direction_degrees,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateState {
    pub season: Season,
    pub weather: WeatherCondition,
    pub wind: Wind,
    pub temperature_celsius: f64,
}

impl ClimateState {
    pub fn default_spring() -> Self {
        Self {
            season: Season::Spring,
            weather: WeatherCondition::Clear,
            wind: Wind::new(12.0, 90.0),
            temperature_celsius: 20.0,
        }
    }

    /// Computes realistic spatial micro-climate temperature based on Environmental Lapse Rate
    /// (-6.5°C per 1,000m elevation) and Köppen-Geiger latitude gradient
    pub fn local_temperature(&self, elevation_meters: f64, latitude_y: u32, map_height: u32) -> f64 {
        // 1. Environmental Lapse Rate: -0.0065°C per meter (-6.5°C per 1,000m)
        let lapse_cooling = -0.0065 * elevation_meters.max(0.0);

        // 2. Latitude Gradient:
        // y close to 0 = Northern subpolar/temperate zone (cooler)
        // y in middle/south = Equatorial & tropical zone (warmer)
        let h = map_height.max(1) as f64;
        let lat_fraction = (latitude_y as f64 / h).clamp(0.0, 1.0);
        // Equator warmth bias in middle/south (+4.0°C), polar cooling towards north (-5.0°C)
        let lat_offset = (lat_fraction * 9.0) - 5.0;

        self.temperature_celsius + lapse_cooling + lat_offset
    }

    /// Whether this spatial coordinate experiences tropical perennial growth (no freeze)
    pub fn is_tropical_zone(&self, latitude_y: u32, map_height: u32) -> bool {
        let h = map_height.max(1) as f64;
        (latitude_y as f64 / h) >= 0.55
    }

    /// Spatial growth multiplier combining season, weather, and Köppen-Geiger zonality
    pub fn spatial_growth_multiplier(&self, elevation_meters: f64, latitude_y: u32, map_height: u32) -> f64 {
        let is_trop = self.is_tropical_zone(latitude_y, map_height);
        let season_mult = if is_trop {
            // Tropical zone: Monsoon wet/dry cycle rather than freezing winter
            match self.season {
                Season::Winter => 0.9, // Mild dry monsoon, vegetation remains alive!
                Season::Autumn => 1.1,
                _ => 1.3,              // Warm monsoon wet season
            }
        } else {
            self.season.growth_multiplier()
        };

        // Alpine high-altitude freeze suppression:
        let altitude_penalty = if elevation_meters > 1500.0 { 0.4 } else { 1.0 };
        season_mult * self.weather.growth_impact() * altitude_penalty
    }
}

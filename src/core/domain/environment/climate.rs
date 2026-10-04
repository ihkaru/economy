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
}

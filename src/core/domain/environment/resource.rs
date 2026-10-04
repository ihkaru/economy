use serde::{Deserialize, Serialize};
use crate::core::domain::environment::climate::ClimateState;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::spatial::coordinate::GeoCoordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegenerationPace {
    /// Fast: Berries/herbs - rapid turnover, daily/weekly replenishment
    Fast,
    /// Medium: Wild grain - seasonal crop replenishment cycle
    Medium,
    /// SeasonalAquatic: River fishery - spring spawning surges & density-dependent replenishment
    SeasonalAquatic,
    /// Slow: Ancient forest (timber) - long maturation timescale
    Slow,
    /// Geological: Salt & mineral deposits - tide / evaporation cycle
    Geological,
}

impl RegenerationPace {
    pub fn base_rate(&self) -> f64 {
        match self {
            Self::Fast => 0.08,
            Self::Medium => 0.03,
            Self::SeasonalAquatic => 0.05,
            Self::Slow => 0.005,
            Self::Geological => 0.01,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub id: u64,
    pub name: String,
    pub item_id: ItemId,
    pub location: GeoCoordinate,
    pub current_stock: u32,
    pub max_stock: u32,
    pub base_regeneration_rate: f64,
    pub base_decay_rate: f64,
    pub calories_per_unit: f64,
    pub is_edible: bool,
    /// Biological maturity percentage from 0.0 (seedling/depleted) to 1.0 (fully ripe/prime biomass)
    pub maturity: f64,
    /// Natural renewal timescale category
    pub pace: RegenerationPace,
    /// Flexible JSON attributes (e.g. coordinates, sensitivity to storm/drought)
    pub metadata: serde_json::Value,
}

impl ResourceNode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        name: impl Into<String>,
        item_id: ItemId,
        location: GeoCoordinate,
        initial_stock: u32,
        max_stock: u32,
        base_regeneration_rate: f64,
        base_decay_rate: f64,
        calories_per_unit: f64,
        metadata: serde_json::Value,
    ) -> Self {
        let maturity = if max_stock > 0 {
            (initial_stock as f64 / max_stock as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let pace = match item_id {
            ItemId::BERRIES => RegenerationPace::Fast,
            ItemId::FISH => RegenerationPace::SeasonalAquatic,
            ItemId::TIMBER => RegenerationPace::Slow,
            ItemId::SALT => RegenerationPace::Geological,
            _ => RegenerationPace::Medium,
        };

        Self {
            id,
            name: name.into(),
            item_id,
            location,
            current_stock: initial_stock,
            max_stock,
            base_regeneration_rate,
            base_decay_rate,
            calories_per_unit,
            is_edible: calories_per_unit > 0.0,
            maturity,
            pace,
            metadata,
        }
    }

    pub fn with_pace(mut self, pace: RegenerationPace) -> Self {
        self.pace = pace;
        self
    }

    pub fn with_maturity(mut self, maturity: f64) -> Self {
        self.maturity = maturity.clamp(0.0, 1.0);
        self.current_stock = ((self.max_stock as f64) * self.maturity).round() as u32;
        self
    }

    /// Step natural regeneration and decay with spatial Köppen-Geiger micro-climate and elevation
    pub fn step_environment_spatial(&mut self, climate: &ClimateState, elevation: f64, map_height: u32) {
        // Logistic growth with spatial Köppen-Geiger zonality & altitude lapse rate:
        let growth_mult = climate.spatial_growth_multiplier(elevation, self.location.y, map_height);
        let spawning_mult = if self.pace == RegenerationPace::SeasonalAquatic {
            match climate.season {
                crate::core::domain::environment::climate::Season::Spring => 3.0,
                crate::core::domain::environment::climate::Season::Summer => 1.5,
                crate::core::domain::environment::climate::Season::Autumn => 1.0,
                crate::core::domain::environment::climate::Season::Winter => 0.3,
            }
        } else {
            1.0
        };
        let r = self.pace.base_rate() * growth_mult * spawning_mult;

        let seed_factor = self.maturity.max(0.08);
        let growth_delta = r * seed_factor * (1.0 - self.maturity);

        // Decay calculation: base * season * weather * wind vulnerability
        let wind_penalty = if climate.wind.speed_kmh > 40.0 { 1.5 } else { 1.0 };
        let decay_mult = climate.season.decay_multiplier() * climate.weather.decay_impact() * wind_penalty;
        let decay_delta = (self.base_decay_rate / (self.max_stock as f64).max(1.0)) * decay_mult * self.maturity;

        self.maturity = (self.maturity + growth_delta - decay_delta).clamp(0.0, 1.0);
        self.current_stock = ((self.max_stock as f64) * self.maturity).round() as u32;
    }

    /// Step natural regeneration and decay based on environmental factors and maturity curve
    pub fn step_environment(&mut self, climate: &ClimateState) {
        self.step_environment_spatial(climate, 100.0, 50);
    }

    /// Agent harvesting interaction with maturity quality curve and tool efficiency multiplier
    pub fn harvest(&mut self, desired_amount: u32, efficiency_mult: f64) -> u32 {
        if self.current_stock == 0 || self.maturity < 0.05 {
            return 0;
        }

        // Maturity quality curve:
        // >= 0.8: prime harvest yield
        // >= 0.4: developing (60% yield)
        // < 0.4: immature / sapling penalty (only 25% yield)
        let maturity_quality = if self.maturity >= 0.8 {
            1.0
        } else if self.maturity >= 0.4 {
            0.6
        } else {
            0.25
        };

        let base_attempt = if desired_amount > 0 {
            ((desired_amount as f64 * efficiency_mult * maturity_quality).round() as u32).max(1)
        } else {
            0
        };
        let actual_harvested = base_attempt.min(self.current_stock);
        if actual_harvested == 0 {
            return 0;
        }

        self.current_stock = self.current_stock.saturating_sub(actual_harvested);
        self.maturity = if self.max_stock > 0 {
            (self.current_stock as f64 / self.max_stock as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        actual_harvested
    }
}

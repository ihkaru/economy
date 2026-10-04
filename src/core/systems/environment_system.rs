use crate::core::domain::environment::climate::{Season, WeatherCondition, Wind};
use crate::core::domain::time::{Tick, TickDuration};
use crate::core::ports::environment_store::EnvironmentStorePort;
use crate::core::ports::rng_port::RngPort;

#[derive(Default)]
pub struct EnvironmentSystem;

impl EnvironmentSystem {
    pub fn new() -> Self {
        Self
    }

    /// Progress climate, season, wind, and resource nodes
    pub fn step(
        &mut self,
        current_tick: Tick,
        tick_duration: TickDuration,
        env_store: &mut dyn EnvironmentStorePort,
        rng: &mut dyn RngPort,
    ) {
        let fractional_years = tick_duration.fractional_years();
        let total_years_elapsed = current_tick.0 as f64 * fractional_years;

        // 1. Season cycle (4 seasons per year)
        let season_fraction = (total_years_elapsed % 1.0) * 4.0;
        let new_season = match season_fraction as usize {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        };

        // 2. Weather transition (Markov / stochastic transition)
        let weather_roll = rng.next_f64();
        let new_weather = if weather_roll < 0.60 {
            WeatherCondition::Clear
        } else if weather_roll < 0.85 {
            WeatherCondition::Rain
        } else if weather_roll < 0.95 {
            WeatherCondition::Storm
        } else {
            WeatherCondition::Drought
        };

        // 3. Wind speed & direction
        let wind_speed = match new_weather {
            WeatherCondition::Clear => rng.gen_range_f64(5.0, 20.0),
            WeatherCondition::Rain => rng.gen_range_f64(15.0, 35.0),
            WeatherCondition::Storm => rng.gen_range_f64(40.0, 95.0),
            WeatherCondition::Drought => rng.gen_range_f64(2.0, 15.0),
        };
        let wind_direction = rng.gen_range_f64(0.0, 360.0);

        // 4. Temperature
        let base_temp = match new_season {
            Season::Spring => 20.0,
            Season::Summer => 32.0,
            Season::Autumn => 18.0,
            Season::Winter => 5.0,
        };
        let temp_variation = rng.gen_range_f64(-3.0, 3.0);

        // Update climate in store
        {
            let climate = env_store.climate_mut();
            climate.season = new_season;
            climate.weather = new_weather;
            climate.wind = Wind::new(wind_speed, wind_direction);
            climate.temperature_celsius = base_temp + temp_variation;
        }

        // 5. Update resource nodes (growth & decay)
        let climate_snapshot = env_store.climate().clone();
        for node in env_store.nodes_mut() {
            node.step_environment(&climate_snapshot);
        }
    }
}

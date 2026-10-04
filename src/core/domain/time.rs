use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TickDuration {
    Hour,
    Day,
    Month,
    Year,
    CustomSeconds(u64),
}

impl TickDuration {
    /// Number of hours represented by one tick (useful for aging and seasonal cycles)
    pub fn approximate_hours(&self) -> f64 {
        match self {
            Self::Hour => 1.0,
            Self::Day => 24.0,
            Self::Month => 24.0 * 30.0,
            Self::Year => 24.0 * 365.0,
            Self::CustomSeconds(secs) => *secs as f64 / 3600.0,
        }
    }

    /// Number of years represented by one tick (fractional)
    pub fn fractional_years(&self) -> f64 {
        self.approximate_hours() / (24.0 * 365.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Tick(pub u64);

impl Tick {
    pub const ZERO: Tick = Tick(0);

    pub fn next(&self) -> Tick {
        Tick(self.0 + 1)
    }

    pub fn advance(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RunId(pub String);

impl RunId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }

    pub fn generate_random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationClock {
    current_tick: Tick,
    duration_per_tick: TickDuration,
}

impl SimulationClock {
    pub fn new(duration_per_tick: TickDuration) -> Self {
        Self {
            current_tick: Tick::ZERO,
            duration_per_tick,
        }
    }

    pub fn current_tick(&self) -> Tick {
        self.current_tick
    }

    pub fn duration_per_tick(&self) -> TickDuration {
        self.duration_per_tick
    }

    pub fn advance_tick(&mut self) -> Tick {
        self.current_tick.advance();
        self.current_tick
    }

    pub fn elapsed_years(&self) -> f64 {
        self.current_tick.0 as f64 * self.duration_per_tick.fractional_years()
    }
}

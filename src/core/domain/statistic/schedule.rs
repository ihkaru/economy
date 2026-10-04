use serde::{Deserialize, Serialize};
use crate::core::domain::time::{SimulationClock, Tick};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseSchedule {
    /// Every N hours
    EveryHours(u32),
    /// Every N days
    EveryDays(u32),
    /// Every N months (1 = monthly, 3 = quarterly, 4 = tertile, 6 = semiannual)
    EveryMonths(u32),
    /// Specific calendar day of the month (e.g., 1st or 15th of every month)
    DayOfMonth(u8),
    /// Every N years (annual)
    EveryYears(u32),
    /// Every N raw simulation ticks
    EveryTicks(u64),
}

impl ReleaseSchedule {
    /// Determines whether a statistical release is due on the current tick
    pub fn is_due(&self, current_tick: Tick, clock: &SimulationClock) -> bool {
        let duration = clock.duration_per_tick();
        let total_hours = current_tick.0 as f64 * duration.approximate_hours();
        let total_days = total_hours / 24.0;
        let total_months = total_days / 30.0;
        let total_years = total_days / 365.0;

        match self {
            Self::EveryHours(h) => {
                let h_u64 = *h as u64;
                (total_hours.round() as u64).is_multiple_of(h_u64)
            }
            Self::EveryDays(d) => {
                let d_u64 = *d as u64;
                (total_days.round() as u64).is_multiple_of(d_u64)
            }
            Self::EveryMonths(m) => {
                let m_u64 = *m as u64;
                (total_months.round() as u64).is_multiple_of(m_u64)
            }
            Self::DayOfMonth(dom) => {
                // Approximate 30-day calendar month cycle (1..=30)
                let day_in_month = ((total_days.round() as u64) % 30) + 1;
                day_in_month == (*dom as u64)
            }
            Self::EveryYears(y) => {
                let y_u64 = *y as u64;
                (total_years.round() as u64).is_multiple_of(y_u64)
            }
            Self::EveryTicks(t) => current_tick.0.is_multiple_of(*t),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::EveryHours(h) => format!("Every {} hour(s)", h),
            Self::EveryDays(d) => format!("Every {} day(s)", d),
            Self::EveryMonths(m) => match m {
                1 => "Monthly".to_string(),
                3 => "Quarterly (Every 3 months)".to_string(),
                4 => "Tertile (Every 4 months)".to_string(),
                6 => "Semiannual (Every 6 months)".to_string(),
                other => format!("Every {} month(s)", other),
            },
            Self::DayOfMonth(dom) => format!("Day {} of each month", dom),
            Self::EveryYears(y) => format!("Every {} year(s)", y),
            Self::EveryTicks(t) => format!("Every {} tick(s)", t),
        }
    }
}

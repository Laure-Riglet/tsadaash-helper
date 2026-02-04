use chrono::{DateTime, Utc, Weekday, Month};

#[derive(Debug, Clone)]
pub enum DaySetting {
    EveryNDays(u8),
    SpecificDaysWeek(Vec<Weekday>),
    SpecificDaysMonth(Vec<u8>),
    SpecificDaysMonthEnd(Vec<u8>),
    DaysPerWeek(Vec<u8>),
    DaysPerFortnight(Vec<u8>),
    DaysPerMonth(Vec<u8>),
    DaysPerYear(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct DayConfig {
    pub rep_per_day: u8,
    pub day_setting: DaySetting,
}

#[derive(Debug, Clone)]
pub struct WeekConfig {
    pub rep_per_week: u8,
    pub week_start: Weekday,
}

#[derive(Debug, Clone)]
pub struct MonthConfig {
    pub rep_per_month: u8,
    pub month_start: Month,
}

#[derive(Debug, Clone)]
pub struct YearConfig {
    pub rep_per_year: u8,
    pub year_start: Month,
}

#[derive(Debug, Clone)]
pub struct CustomConfig {
    pub dates: Vec<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UniqueConfig {
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum PeriodicityConfig {
    Day(DayConfig),
    Week(WeekConfig),
    Month(MonthConfig),
    Year(YearConfig),
    Unique(UniqueConfig),
    Custom(CustomConfig),
}

#[derive(Debug, Clone)]
pub struct Periodicity {
    pub config: PeriodicityConfig,
}
use chrono::{DateTime, Month, Utc, Weekday};

// ---------------------------------------------------------- //
// DAY STUFFS
// Task that occur on specific days of the week or month or 
// every N days with no specific focus on which days of the 
// week or month
// ---------------------------------------------------------- //

#[derive(Debug, Clone)]
pub enum SpecificDayMonthSetting {
    MonthWeekFromFirst(u8), // 1-4 (first to fifth week of the month)
    MonthWeekFromLast(u8),  // 1-4 (last to fourth last week of the month)
}

#[derive(Debug, Clone)]
pub enum SpecificDayMonth {
    Weekday(Weekday),
    MonthWeek(SpecificDayMonthSetting),
}

#[derive(Debug, Clone)]
pub enum DaySetting {
    // GENERAL SETTINGS
    // Specify a regular pattern without focusing on specific days
    EveryDay(),     // tasks that occur every day
    EveryNDays(u8), // 1-366 (arbitrary limit) - tasks that occur every N days

    // FOCUSED SETTINGS
    // Focused on specific days of the week or month
    SpecificDaysWeek(Vec<Weekday>), // max 7 elements - tasks that occur on specific days of the week
    SpecificDaysMonthFromFirst(Vec<u8>), // 0-30 - tasks that occur on specific days of the month (from start, e.g., 0 --> 1 + 0 --> first day of month)
    SpecificDaysMonthFromLast(Vec<u8>), // 0-30 - tasks that occur on specific days of the month (from end, e.g., 3 --> last day - 3)
    SpecificNthWeekdaysMonth(Vec<SpecificDayMonth>), // tasks that occur on specific nth weekdays of the month (e.g., first Monday & third Friday, etc.)

    // COUNT SETTINGS
    // Focused on counting occurrences within a time frame, without specifying exact days
    DaysPerWeek(u8), // 1-7 - tasks that occur a certain number of times per week (on any days)
    DaysPerFortnight(u8), // 1-14 - tasks that occur a certain number of times per fortnight (on any days)
    DaysPerMonth(u8), // 1-31 - tasks that occur a certain number of times per month (on any days)
    DaysPerYear(u8),  // 1-366 - tasks that occur a certain number of times per year (on any days)
}

// ---------------------------------------------------------- //
// WEEK STUFFS 
// Task that occur on specific weeks of the month or every 
// N weeks with no specific focus on which days of the week
// ---------------------------------------------------------- //

#[derive(Debug, Clone)]
pub enum WeekSetting {
    EveryWeek(),                            // tasks that occur every week
    EveryNWeeks(u8),                        // 1-52 (arbitrary limit) - tasks that occur every N weeks
    SpecificWeeksOfMonthFromFirst(Vec<u8>), // 1-4 - tasks that occur on specific weeks of the month (from start)
    SpecificWeeksOfMonthFromLast(Vec<u8>),  // 1-4 - tasks that occur on specific weeks of the month (from end)
    WeeksPerMonth(u8),                      // 1-4 - tasks that occur a certain number of times per month (on any weeks)
}

#[derive(Debug, Clone)]
pub enum MonthSetting {
    EveryMonth(),               // tasks that occur every month (default)
    EveryNMonths(u8),           // 1-12 (arbitrary limit) - tasks that occur every N months
    SpecificMonths(Vec<Month>), // max 12 elements - tasks that occur on specific months of the year
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
    pub week_setting: WeekSetting,
}

#[derive(Debug, Clone)]
pub struct MonthConfig {
    pub rep_per_month: u8,
    pub month_setting: MonthSetting,
}

#[derive(Debug, Clone)]
pub struct YearConfig {
    pub rep_per_year: u8,
    pub year_start: Month,
}

#[derive(Debug, Clone)]
pub struct CustomConfig {
    pub dates: Vec<DateTime<Utc>>, // Specific dates for the task to occur, without a regular pattern
}

#[derive(Debug, Clone)]
pub struct UniqueConfig {
    pub date: DateTime<Utc>, // A one-time task that occurs on a specific date, without any repetition
}

#[derive(Debug, Clone)]
pub enum PeriodicityConfig {
    Day(DayConfig),
    Week(WeekConfig),
    Month(MonthConfig),
    Unique(UniqueConfig),
    Custom(CustomConfig),
    Year(YearConfig),
}

#[derive(Debug, Clone)]
pub struct Periodicity {
    pub config: PeriodicityConfig,
    // Keep the struct extensible for future fields like time of day, exceptions, etc.
    pub timeframe: Option<(DateTime<Utc>, DateTime<Utc>)>, // Optional time frame for the periodicity (start and end) / Not OK - we should handle this in another struct like TimeFrameConfig.
}

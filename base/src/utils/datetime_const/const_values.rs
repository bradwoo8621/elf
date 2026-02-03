pub const HALF_YEAR_FIRST: u8 = 1;
pub const HALF_YEAR_SECOND: u8 = 2;

pub const QUARTER_FIRST: u8 = 1;
pub const QUARTER_SECOND: u8 = 2;
pub const QUARTER_THIRD: u8 = 3;
pub const QUARTER_FOURTH: u8 = 4;

pub const JANUARY: u8 = 1;
pub const FEBRUARY: u8 = 2;
pub const MARCH: u8 = 3;
pub const APRIL: u8 = 4;
pub const MAY: u8 = 5;
pub const JUNE: u8 = 6;
pub const JULY: u8 = 7;
pub const AUGUST: u8 = 8;
pub const SEPTEMBER: u8 = 9;
pub const OCTOBER: u8 = 10;
pub const NOVEMBER: u8 = 11;
pub const DECEMBER: u8 = 12;

pub const HALF_MONTH_FIRST: u8 = 1;
pub const HALF_MONTH_SECOND: u8 = 2;

pub const TEN_DAYS_FIRST: u8 = 1;
pub const TEN_DAYS_SECOND: u8 = 2;
pub const TEN_DAYS_THIRD: u8 = 3;

/// first week less than 7 days, otherwise week of year starts from 1
pub const WEEK_OF_YEAR_FIRST_SHORT: u8 = 0;
pub const WEEK_OF_YEAR_FIRST: u8 = 1;
pub const WEEK_OF_YEAR_LAST: u8 = 53;

/// first week less than 7 days, otherwise week of month starts from 1
pub const WEEK_OF_MONTH_FIRST_SHORT: u8 = 0;
pub const WEEK_OF_MONTH_FIRST: u8 = 1;
pub const WEEK_OF_MONTH_LAST: u8 = 5;

pub const HALF_WEEK_FIRST: u8 = 1;
pub const HALF_WEEK_SECOND: u8 = 2;

pub const DAY_OF_MONTH_MIN: u8 = 1;
pub const DAY_OF_MONTH_MAX: u8 = 31;

pub const SUNDAY: u8 = 1;
pub const MONDAY: u8 = 2;
pub const TUESDAY: u8 = 3;
pub const WEDNESDAY: u8 = 4;
pub const THURSDAY: u8 = 5;
pub const FRIDAY: u8 = 6;
pub const SATURDAY: u8 = 7;

pub const DAY_KIND_WORKDAY: u8 = 1;
pub const DAY_KIND_WEEKEND: u8 = 2;
pub const DAY_KIND_HOLIDAY: u8 = 3;

pub const HOUR_KIND_WORKTIME: u8 = 1;
pub const HOUR_KIND_OFF_HOURS: u8 = 2;
pub const HOUR_KIND_SLEEPING_TIME: u8 = 3;

pub const AM: u8 = 1;
pub const PM: u8 = 2;

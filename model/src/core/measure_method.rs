use crate::ModelErrorCode;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum MeasureMethod {
    // address related
    Continent,
    Region,
    Country,
    Province,
    City,
    District,
    Floor,
    ResidenceType,
    ResidentialArea,

    // time related
    Year,
    HalfYear,
    Quarter,
    Month,
    HalfMonth,
    TenDays,
    WeekOfYear,
    WeekOfMonth,
    HalfWeek,
    DayOfMonth,
    DayOfWeek,
    DayKind,
    Hour,
    HourKind,
    AmPm,

    // individual related
    Gender,
    Occupation,
    Age,
    Religion,
    Nationality,

    // organization related
    BizTrade,
    BizScale,

    // boolean
    Boolean,

    // enumeration
    Enum,
}

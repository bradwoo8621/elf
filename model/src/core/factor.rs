use crate::{BaseDataModel, EnumId, Storable};
use std::cmp::PartialEq;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, PartialEq, StrEnum)]
pub enum FactorType {
    Sequence,
    Number,
    /// 0 & positive
    Unsigned,
    Text,
    // address
    Address,
    Continent,
    Region,
    Country,
    Province,
    City,
    District,
    Road,
    Community,
    Floor,
    ResidenceType,
    ResidentialArea,
    // contact electronic
    Email,
    Phone,
    Mobile,
    Fax,
    // date time related
    /// YYYY-MM-DD HH:mm:ss
    Datetime,
    /// YYYY-MM-DD HH:mm:ss.SSS
    FullDatetime,
    /// YYYY-MM-DD
    Date,
    /// HH:mm:ss
    Time,
    /// 4 digits
    Year,
    /// 1: first half, 2: second half
    HalfYear,
    /// 1 - 4
    Quarter,
    /// 1 - 12
    Month,
    /// 1: first half, 2: second half
    HalfMonth,
    /// 1, 2, 3
    TenDays,
    /// 0 (the partial week that precedes the first Sunday of the year) - 53 (leap year)
    WeekOfYear,
    /// 0 (the partial week that precedes the first Sunday of the year) - 5
    WeekOfMonth,
    /// 1: first half, 2: second half
    HalfWeek,
    /// 1 - 31, according to month/year
    DayOfMonth,
    /// 1 (Sunday) - 7 (Saturday)
    DayOfWeek,
    /// 1: workday, 2: weekend, 3: holiday
    DayKind,
    /// 0 - 23
    Hour,
    /// 1: work time, 2: off hours, 3: sleeping time
    HourKind,
    /// 0 - 59
    Minute,
    /// 0 - 59
    Second,
    /// 0 - 999
    Millisecond,
    /// 1, 2
    AmPm,
    // individual
    Gender,
    Occupation,
    /// YYYY-MM-DD
    DateOfBirth,
    Age,
    IdNo,
    Religion,
    Nationality,
    // organization
    BizTrade,
    BizScale,
    Boolean,
    Enum,
    Object,
    Array,
}

#[derive(PartialEq)]
pub enum FactorTypeCategory {
    Text,
    TextLike,
    EnumText,
    Numeric,
    DatetimeNumeric,
    FullDatetime,
    Datetime,
    Date,
    Time,
    Boolean,
    Complex,
}

impl FactorType {
    pub fn category(&self) -> FactorTypeCategory {
        match self {
            FactorType::Text => FactorTypeCategory::Text,
            FactorType::Address
            | FactorType::Road
            | FactorType::Community
            | FactorType::Email
            | FactorType::Phone
            | FactorType::Mobile
            | FactorType::Fax
            | FactorType::Occupation
            | FactorType::IdNo => FactorTypeCategory::TextLike,
            FactorType::Continent
            | FactorType::Region
            | FactorType::Country
            | FactorType::Province
            | FactorType::City
            | FactorType::District
            | FactorType::ResidenceType
            | FactorType::Gender
            | FactorType::Religion
            | FactorType::Nationality
            | FactorType::BizTrade
            | FactorType::Enum => FactorTypeCategory::EnumText,
            FactorType::Sequence
            | FactorType::Number
            | FactorType::Unsigned
            | FactorType::Floor
            | FactorType::ResidentialArea
            | FactorType::Age
            | FactorType::BizScale => FactorTypeCategory::Numeric,
            FactorType::Year
            | FactorType::HalfYear
            | FactorType::Quarter
            | FactorType::Month
            | FactorType::HalfMonth
            | FactorType::TenDays
            | FactorType::WeekOfYear
            | FactorType::WeekOfMonth
            | FactorType::HalfWeek
            | FactorType::DayOfMonth
            | FactorType::DayOfWeek
            | FactorType::DayKind
            | FactorType::Hour
            | FactorType::HourKind
            | FactorType::Minute
            | FactorType::Second
            | FactorType::Millisecond
            | FactorType::AmPm => FactorTypeCategory::DatetimeNumeric,
            FactorType::FullDatetime => FactorTypeCategory::FullDatetime,
            FactorType::Datetime => FactorTypeCategory::Datetime,
            FactorType::Date | FactorType::DateOfBirth => FactorTypeCategory::Date,
            FactorType::Time => FactorTypeCategory::Time,
            FactorType::Boolean => FactorTypeCategory::Boolean,
            FactorType::Object | FactorType::Array => FactorTypeCategory::Complex,
        }
    }
}

#[derive(Display, Serde, StrEnum)]
pub enum FactorIndexGroup {
    #[display = ""]
    EMPTY,
    #[display = "i-1"]
    Index1,
    #[display = "i-2"]
    Index2,
    #[display = "i-3"]
    Index3,
    #[display = "i-4"]
    Index4,
    #[display = "i-5"]
    Index5,
    #[display = "i-6"]
    Index6,
    #[display = "i-7"]
    Index7,
    #[display = "i-8"]
    Index8,
    #[display = "i-9"]
    Index9,
    #[display = "i-10"]
    Index10,
    #[display = "u-1"]
    UniqueIndex1,
    #[display = "u-2"]
    UniqueIndex2,
    #[display = "u-3"]
    UniqueIndex3,
    #[display = "u-4"]
    UniqueIndex4,
    #[display = "u-5"]
    UniqueIndex5,
    #[display = "u-6"]
    UniqueIndex6,
    #[display = "u-7"]
    UniqueIndex7,
    #[display = "u-8"]
    UniqueIndex8,
    #[display = "u-9"]
    UniqueIndex9,
    #[display = "u-10"]
    UniqueIndex10,
}

#[derive(Display, Serde, StrEnum)]
#[pattern = "kebab-upper"]
pub enum FactorEncryptMethod {
    #[display = "none"]
    None,
    Aes256Pkcs5Padding,
    Md5,
    Sha256,
    MaskMail,
    #[display = "MASK-CENTER-3"]
    MaskCenter3,
    #[display = "MASK-CENTER-5"]
    MaskCenter5,
    #[display = "MASK-LAST-3"]
    MaskLast3,
    #[display = "MASK-LAST-6"]
    MaskLast6,
    MaskDay,
    MaskMonth,
    MaskMonthDay,
}

pub type FactorId = String;

#[adapt_model(storable)]
pub struct Factor {
    pub factor_id: Option<FactorId>,
    pub r#type: Option<FactorType>,
    pub name: Option<String>,
    pub enum_id: Option<EnumId>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub flatten: Option<bool>,
    pub index_group: Option<FactorIndexGroup>,
    pub encrypt: Option<FactorEncryptMethod>,
    pub precision: Option<String>,
}

impl Factor {
    pub fn is_flatten(&self) -> bool {
        self.flatten.unwrap_or_else(|| false)
    }

    pub fn is_date_or_time(&self) -> bool {
        match self.r#type {
            Some(FactorType::Date)
            | Some(FactorType::Time)
            | Some(FactorType::Datetime)
            | Some(FactorType::FullDatetime)
            | Some(FactorType::DateOfBirth) => true,
            _ => false,
        }
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }
}

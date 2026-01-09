use crate::{
    Auditable, BaseDataModel, EnumId, MeasureMethod, ModelErrorCode, OptimisticLock, Storable,
    TenantBasedTuple, TenantId, Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};
use serde::{Deserialize, Serialize};

#[derive(Display, Serde, StrEnum)]
pub enum BucketType {
    Value,
    ValueMeasure,
    CategoryMeasure,
    EnumMeasure,
}

#[derive(Display, Serde, StrEnum)]
pub enum RangeBucketValueIncluding {
    IncludeMin,
    IncludeMax,
}

#[adapt_model(storable)]
pub struct NumericSegmentValue {
    pub min: Option<String>,
    pub max: Option<String>,
}

#[adapt_model(storable)]
pub struct NumericValueSegment {
    pub name: Option<String>,
    pub value: Option<NumericSegmentValue>,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct NumericValueBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    pub include: Option<RangeBucketValueIncluding>,
    pub segments: Option<Vec<NumericValueSegment>>,
}

impl NumericValueBucket {
    pub fn init() -> Self {
        Self::new().r#type(BucketType::Value)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::NumericValue(self)
    }
}

#[adapt_model(opt_lock, tenant_based)]
pub struct NumericValueMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    /// can be [MeasureMethod::Floor], [MeasureMethod::ResidentialArea], [MeasureMethod::Age] or [MeasureMethod::BizScale]
    pub measure: Option<MeasureMethod>,
    pub include: Option<RangeBucketValueIncluding>,
    pub segments: Option<Vec<NumericValueSegment>>,
}

impl NumericValueMeasureBucket {
    pub fn init() -> Self {
        Self::new().r#type(BucketType::ValueMeasure)
    }

    pub fn with_floor() -> Self {
        Self::init().measure(MeasureMethod::Floor)
    }

    pub fn with_residential_area() -> Self {
        Self::init().measure(MeasureMethod::ResidentialArea)
    }

    pub fn with_age() -> Self {
        Self::init().measure(MeasureMethod::Age)
    }

    pub fn with_biz_scale() -> Self {
        Self::init().measure(MeasureMethod::BizScale)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::NumericValueMeasure(self)
    }
}

pub const OTHER_CATEGORY_SEGMENT_VALUE: &'static str = "&others";
/// [OTHER_CATEGORY_SEGMENT_VALUE]
pub type CategorySegmentValue = Vec<String>;

#[adapt_model(storable)]
pub struct CategorySegment {
    pub name: Option<String>,
    pub value: Option<CategorySegmentValue>,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct CategoryMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    /// can be one of following:
    /// - [MeasureMethod::Continent],
    /// - [MeasureMethod::Region],
    /// - [MeasureMethod::Country],
    /// - [MeasureMethod::Province],
    /// - [MeasureMethod::City],
    /// - [MeasureMethod::District],
    /// - [MeasureMethod::ResidenceType],
    /// - [MeasureMethod::Gender],
    /// - [MeasureMethod::Occupation],
    /// - [MeasureMethod::Religion],
    /// - [MeasureMethod::Nationality],
    /// - [MeasureMethod::BizTrade],
    /// - [MeasureMethod::Boolean],
    pub measure: Option<MeasureMethod>,
    pub segments: Option<Vec<CategorySegment>>,
}

impl CategoryMeasureBucket {
    pub fn init() -> Self {
        Self::new().r#type(BucketType::CategoryMeasure)
    }

    pub fn with_continent() -> Self {
        Self::init().measure(MeasureMethod::Continent)
    }

    pub fn with_region() -> Self {
        Self::init().measure(MeasureMethod::Region)
    }

    pub fn with_country() -> Self {
        Self::init().measure(MeasureMethod::Country)
    }

    pub fn with_province() -> Self {
        Self::init().measure(MeasureMethod::Province)
    }

    pub fn with_city() -> Self {
        Self::init().measure(MeasureMethod::City)
    }

    pub fn with_district() -> Self {
        Self::init().measure(MeasureMethod::District)
    }

    pub fn with_residence_type() -> Self {
        Self::init().measure(MeasureMethod::ResidenceType)
    }

    pub fn with_gender() -> Self {
        Self::init().measure(MeasureMethod::Gender)
    }

    pub fn with_occupation() -> Self {
        Self::init().measure(MeasureMethod::Occupation)
    }

    pub fn with_religion() -> Self {
        Self::init().measure(MeasureMethod::Religion)
    }

    pub fn with_nationality() -> Self {
        Self::init().measure(MeasureMethod::Nationality)
    }

    pub fn with_biz_trade() -> Self {
        Self::init().measure(MeasureMethod::BizTrade)
    }

    pub fn with_boolean() -> Self {
        Self::init().measure(MeasureMethod::Boolean)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::CategoryMeasure(self)
    }
}

#[adapt_model(opt_lock, tenant_based)]
pub struct EnumMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    pub measure: Option<MeasureMethod>,
    pub segments: Option<Vec<CategorySegment>>,
    pub enum_id: Option<EnumId>,
}

impl EnumMeasureBucket {
    pub fn init() -> Self {
        Self::new()
            .r#type(BucketType::EnumMeasure)
            .measure(MeasureMethod::Enum)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::EnumMeasure(self)
    }
}

pub type BucketId = String;

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum Bucket {
    NumericValue(NumericValueBucket),
    NumericValueMeasure(NumericValueMeasureBucket),
    CategoryMeasure(CategoryMeasureBucket),
    EnumMeasure(EnumMeasureBucket),
}

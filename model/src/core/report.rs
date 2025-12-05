use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, Chart, ConnectedSpaceId, DataResultSet, GraphicRect, LastVisit,
    ParameterJoint, StdErrCode, StdErrorCode, StdR, Storable, SubjectDatasetColumnId, SubjectId,
    TenantId, UserBasedTuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum ReportIndicatorArithmetic {
    None,
    Count,
    #[display = "distinct_count"]
    DistinctCount,
    Sum,
    Avg,
    Max,
    Min,
}

#[adapt_model(storable)]
pub struct ReportIndicator {
    pub column_id: Option<SubjectDatasetColumnId>,
    pub name: Option<String>,
    pub arithmetic: Option<ReportIndicatorArithmetic>,
}

#[adapt_model(storable)]
pub struct ReportDimension {
    pub column_id: Option<SubjectDatasetColumnId>,
    pub name: Option<String>,
}

#[derive(Display, Serde, StrEnum)]
pub enum ReportFunnelType {
    Numeric,
    Date,
    Year,
    HalfYear,
    Quarter,
    Month,
    HalfMonth,
    TenDays,
    WeekOfMonth,
    HalfWeek,
    DayKind,
    DayOfWeek,
    Hour,
    HourKind,
    AmPm,
    Enum,
}

pub type ReportFunnelId = String;

#[adapt_model(storable)]
pub struct ReportFunnel {
    pub funnel_id: Option<ReportFunnelId>,
    pub column_id: Option<SubjectDatasetColumnId>,
    pub r#type: Option<ReportFunnelType>,
    pub range: Option<bool>,
    pub enabled: Option<bool>,
    pub values: Option<Vec<Option<String>>>,
}

pub type ReportId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct Report {
    pub report_id: Option<ReportId>,
    pub name: Option<String>,
    pub subject_id: Option<SubjectId>,
    pub connect_id: Option<ConnectedSpaceId>,
    pub funnels: Option<Vec<ReportFunnel>>,
    pub indicators: Option<Vec<ReportIndicator>>,
    pub dimensions: Option<Vec<ReportDimension>>,
    pub description: Option<String>,
    pub rect: Option<GraphicRect>,
    pub chart: Option<Chart>,
    pub simulating: Option<bool>,
    pub simulate_data: Option<DataResultSet>,
    /// base64
    pub simulate_thumbnail: Option<String>,
    pub filters: Option<ParameterJoint>,
}

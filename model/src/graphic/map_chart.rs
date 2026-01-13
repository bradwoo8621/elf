use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType, EChartsGridPositionOnly,
    EChartsTitle, ModelErrorCode, PredefinedChartColorSeries, Storable,
};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum MapChartRegion {
    ChinaL1,
    CyprusL1,
    JapanL1,
    SingaporeL1,
    UsaL1,
}

#[adapt_model(storable)]
pub struct MapChartSettingsSeries {
    pub region: Option<MapChartRegion>,
}

#[adapt_model(storable)]
pub struct MapChartSettings {
    pub series: Option<MapChartSettingsSeries>,
    pub grid: Option<EChartsGridPositionOnly>,
    /// [EChartsSettings]
    /// [EChartsTitleHolder]
    pub title: Option<EChartsTitle>,
    /// [ChartSettings]
    pub border: Option<ChartBorder>,
    pub background_color: Option<ChartColor>,
    pub color_series: Option<PredefinedChartColorSeries>,
    /// [ChartTruncationHolder]
    pub truncation: Option<ChartTruncation>,
}

#[adapt_model(storable)]
pub struct MapChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<MapChartSettings>,
}

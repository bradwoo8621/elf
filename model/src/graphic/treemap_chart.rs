use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType, EChartsGridPositionOnly,
    EChartsTitle, PredefinedChartColorSeries, Storable,
};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TreemapChartSettingsSeries {
    pub roam: Option<bool>,
}

#[adapt_model(storable)]
pub struct TreemapChartSettings {
    pub series: Option<TreemapChartSettingsSeries>,
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
pub struct TreemapChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<TreemapChartSettings>,
}

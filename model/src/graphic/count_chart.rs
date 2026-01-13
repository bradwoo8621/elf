use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartFont, ChartTruncation, ChartType, EChartsTitle,
    PredefinedChartColorSeries, Storable,
};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct CountChartSettingsText {
    pub font: Option<ChartFont>,
    pub format_use_grouping: Option<bool>,
}

#[adapt_model(storable)]
pub struct CountChartSettings {
    pub count_text: Option<CountChartSettingsText>,
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
pub struct CountChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<CountChartSettings>,
}

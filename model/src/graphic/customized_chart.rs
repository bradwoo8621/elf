use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType, EChartsScriptsVars,
    EChartsTitle, PredefinedChartColorSeries, Storable,
};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct CustomizedChartSettings {
    /// [EchartsScriptHolder]
    pub script: Option<String>,
    pub script_vars_defs: Option<String>,
    pub script_vars: Option<EChartsScriptsVars>,
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
pub struct CustomizedChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<CustomizedChartSettings>,
}

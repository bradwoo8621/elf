use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType, EChartsGridPositionOnly,
    EChartsLegend, EChartsTitle, PieChartSettingsLabel, PredefinedChartColorSeries, Storable,
};
use bigdecimal::BigDecimal;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct SunburstChartSettingsSeries {
    pub center_x: Option<BigDecimal>,
    pub center_y: Option<BigDecimal>,
    pub inside_radius: Option<BigDecimal>,
    pub outside_radius: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
}

#[adapt_model(storable)]
pub struct SunburstChartSettings {
    pub series: Option<SunburstChartSettingsSeries>,
    pub grid: Option<EChartsGridPositionOnly>,
    pub label: Option<PieChartSettingsLabel>,
    pub decal: Option<bool>,
    /// [EChartsLegendHolder]
    pub legend: Option<EChartsLegend>,
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
pub struct SunburstChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<SunburstChartSettings>,
}

use crate::{
    BarChartSettingsLabel, BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType,
    EChartsGrid, EChartsLegend, EChartsTitle, EChartsXAxis, EChartsYAxis,
    PredefinedChartColorSeries, Storable,
};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct LineChartSettingsSeries {
    pub smooth: Option<bool>,
    /// [BarChartSettingsSeries]
    pub transform_axis: Option<bool>,
}

#[adapt_model(storable)]
pub struct LineChartSettings {
    series: Option<LineChartSettingsSeries>,
    label: Option<BarChartSettingsLabel>,
    /// [EChartsLegendHolder]
    pub legend: Option<EChartsLegend>,
    /// [EChartsGridHolder]
    pub grid: Option<EChartsGrid>,
    /// [EChartsXAxisHolder]
    pub xaxis: Option<EChartsXAxis>,
    /// [EChartsYAxisHolder]
    pub yaxis: Option<EChartsYAxis>,
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
pub struct LineChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<LineChartSettings>,
}

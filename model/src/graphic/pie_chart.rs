use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartFont, ChartTruncation, ChartType,
    EChartsGridPositionOnly, EChartsHorizontalAlignment, EChartsLegend, EChartsTitle,
    EChartsVerticalAlignment, PredefinedChartColorSeries, Storable,
};
use bigdecimal::BigDecimal;
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum PieRoseType {
    None,
    Radius,
    Area,
}

#[derive(Display, Serde)]
pub enum PieLabelPosition {
    Inside,
    Outside,
    Center,
}

#[derive(Display, Serde)]
pub enum PieLabelAlignTo {
    None,
    #[display = "labelLine"]
    LabelLine,
    Edge,
}

#[adapt_model(storable)]
pub struct PieChartSettingsLabel {
    pub show: Option<bool>,
    pub background_color: Option<ChartColor>,
    pub position: Option<PieLabelPosition>,
    pub align_to: Option<PieLabelAlignTo>,
    pub rotate: Option<BigDecimal>,
    pub gap: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    pub format_use_grouping: Option<bool>,
    pub format_use_percentage: Option<bool>,
    pub value_as_percentage: Option<bool>,
    pub fraction_digits: Option<i32>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct PieChartSettingsSeries {
    pub center_x: Option<BigDecimal>,
    pub center_y: Option<BigDecimal>,
    pub inside_radius: Option<BigDecimal>,
    pub outside_radius: Option<BigDecimal>,
    pub rose_type: Option<PieRoseType>,
    pub show_percentage: Option<bool>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
}

#[adapt_model(storable)]
pub struct PieChartSettings {
    pub series: Option<PieChartSettingsSeries>,
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
pub struct PieChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<PieChartSettings>,
}

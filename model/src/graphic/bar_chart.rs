use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartFont, ChartTruncation, ChartType, EChartsGrid,
    EChartsHorizontalAlignment, EChartsLegend, EChartsTitle, EChartsVerticalAlignment,
    EChartsXAxis, EChartsYAxis, ModelErrorCode, PredefinedChartColorSeries, Storable,
};
use bigdecimal::BigDecimal;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum BarLabelPosition {
    Top,
    Left,
    Right,
    Bottom,
    Inside,
    #[display = "insideLeft"]
    InsideLeft,
    #[display = "insideRight"]
    InsideRight,
    #[display = "insideTop"]
    InsideTop,
    #[display = "insideBottom"]
    InsideBottom,
    #[display = "insideTopLeft"]
    InsideTopLeft,
    #[display = "insideBottomLeft"]
    InsideBottomLeft,
    #[display = "insideTopRight"]
    InsideTopRight,
    #[display = "insideBottomRight"]
    InsideBottomRight,
}

// noinspection DuplicatedCode
#[adapt_model(storable)]
pub struct BarChartSettingsLabel {
    pub show: Option<bool>,
    pub background_color: Option<ChartColor>,
    pub position: Option<BarLabelPosition>,
    pub rotate: Option<BigDecimal>,
    pub gap: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    pub format_use_grouping: Option<bool>,
    pub format_use_percentage: Option<bool>,
    pub value_as_percentage: Option<bool>,
    pub fraction_digits: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct BarChartSettingsSeries {
    pub transform_axis: Option<bool>,
}

#[adapt_model(storable)]
pub struct BarChartSettings {
    pub series: Option<BarChartSettingsSeries>,
    pub label: Option<BarChartSettingsLabel>,
    pub decal: Option<bool>,
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
pub struct BarChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<BarChartSettings>,
}

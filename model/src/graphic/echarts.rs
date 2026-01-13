use crate::{
    BaseDataModel, ChartBorder, ChartBorderStyle, ChartColor, ChartFont, ChartTruncation,
    ModelErrorCode, PredefinedChartColorSeries, Storable,
};
use bigdecimal::BigDecimal;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};
use std::collections::HashMap;

#[adapt_model(storable)]
pub struct EChartsBorderHolder {
    pub border: Option<ChartBorder>,
}

#[adapt_model(storable)]
pub struct EChartsBorderOmitRadius {
    pub color: Option<ChartColor>,
    pub style: Option<ChartBorderStyle>,
    pub width: Option<BigDecimal>,
}

#[adapt_model(storable)]
pub struct EChartsBorderHolderNoRadius {
    pub border: Option<EChartsBorderOmitRadius>,
}

#[adapt_model(storable)]
pub struct EChartsFontHolder {
    pub font: Option<ChartFont>,
}

#[adapt_model(storable)]
pub struct EChartsPosition {
    pub top: Option<BigDecimal>,
    pub right: Option<BigDecimal>,
    pub left: Option<BigDecimal>,
    pub bottom: Option<BigDecimal>,
}

#[adapt_model(storable)]
pub struct EChartsPositionHolder {
    pub position: Option<EChartsPosition>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsHorizontalAlignment {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsVerticalAlignment {
    Auto,
    Top,
    Bottom,
    Middle,
}

#[adapt_model(storable)]
pub struct EChartsAlignmentHolder {
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct EChartsTitleText {
    pub text: Option<String>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
}

#[adapt_model(storable)]
pub struct EChartsTitle {
    pub text: Option<EChartsTitleText>,
    pub subtext: Option<EChartsTitleText>,
    pub background_color: Option<ChartColor>,
    pub padding: Option<BigDecimal>,
    pub item_gap: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsPositionHolder]
    pub position: Option<EChartsPosition>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct EChartsTitleHolder {
    pub title: Option<EChartsTitle>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsLegendOrient {
    Horizontal,
    Vertical,
}

#[adapt_model(storable)]
pub struct EChartsLegend {
    pub show: Option<bool>,
    pub orient: Option<EChartsLegendOrient>,
    pub background_color: Option<ChartColor>,
    pub padding: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsPositionHolder]
    pub position: Option<EChartsPosition>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
}

#[adapt_model(storable)]
pub struct EChartsLegendHolder {
    pub legend: Option<EChartsLegend>,
}

#[adapt_model(storable)]
pub struct EChartsGrid {
    pub show: Option<bool>,
    pub contain_label: Option<bool>,
    pub background_color: Option<ChartColor>,
    /// [EChartsBorderHolderNoRadius]
    pub border: Option<EChartsBorderOmitRadius>,
    /// [EChartsPositionHolder]
    pub position: Option<EChartsPosition>,
}

#[adapt_model(storable)]
pub struct EChartsGridPositionOnly {
    /// [EChartsPositionHolder]
    pub position: Option<EChartsPosition>,
}

#[adapt_model(storable)]
pub struct EChartsGridHolder {
    pub grid: Option<EChartsGrid>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsAxisSplitLineStyle {
    Solid,
    Dashed,
    Dotted,
}

#[adapt_model(storable)]
pub struct EChartsAxisSplitLine {
    pub show: Option<bool>,
    pub color: Option<ChartColor>,
    pub width: Option<BigDecimal>,
    pub style: Option<EChartsAxisSplitLineStyle>,
}

#[adapt_model(storable)]
pub struct EChartsAxisSplitLineHolder {
    pub split_line: Option<EChartsAxisSplitLine>,
}

#[adapt_model(storable)]
pub struct EChartsAxisMinorSplitLineHolder {
    pub minor_split_line: Option<EChartsAxisSplitLine>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsXAxisPosition {
    Top,
    Bottom,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsXAxisType {
    Value,
    Category,
    Time,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsXAxisNameLocation {
    Start,
    Center,
    End,
}

#[adapt_model(storable)]
pub struct EChartsXAxisName {
    pub text: Option<String>,
    pub location: Option<EChartsXAxisNameLocation>,
    pub background_color: Option<ChartColor>,
    pub gap: Option<BigDecimal>,
    pub rotate: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

// noinspection DuplicatedCode
#[adapt_model(storable)]
pub struct EChartsXAxisLabel {
    pub show: Option<bool>,
    pub inside: Option<bool>,
    pub background_color: Option<ChartColor>,
    pub gap: Option<BigDecimal>,
    pub rotate: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct EChartsXAxis {
    //(EChartsAxisSplitLineHolder, EChartsAxisMinorSplitLineHolder):
    pub show: Option<bool>,
    pub position: Option<EChartsXAxisPosition>,
    pub r#type: Option<EChartsXAxisType>,
    pub name: Option<EChartsXAxisName>,
    pub label: Option<EChartsXAxisLabel>,
    pub auto_min: Option<bool>,
    pub min: Option<BigDecimal>,
    pub auto_max: Option<bool>,
    pub max: Option<BigDecimal>,
    /// [EChartsAxisSplitLineHolder]
    pub split_line: Option<EChartsAxisSplitLine>,
    /// [EChartsAxisMinorSplitLineHolder]
    pub minor_split_line: Option<EChartsAxisSplitLine>,
}

#[adapt_model(storable)]
pub struct EChartsXAxisHolder {
    pub xaxis: Option<EChartsXAxis>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsYAxisPosition {
    Left,
    Right,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsYAxisType {
    Value,
    Category,
    Time,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsYAxisNameLocation {
    Start,
    Middle,
    End,
}

#[adapt_model(storable)]
pub struct EChartsYAxisName {
    pub text: Option<String>,
    pub location: Option<EChartsYAxisNameLocation>,
    pub background_color: Option<ChartColor>,
    pub gap: Option<BigDecimal>,
    pub rotate: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

// noinspection DuplicatedCode
#[adapt_model(storable)]
pub struct EChartsYAxisLabel {
    pub show: Option<bool>,
    pub inside: Option<bool>,
    pub background_color: Option<ChartColor>,
    pub gap: Option<BigDecimal>,
    pub rotate: Option<BigDecimal>,
    pub padding: Option<BigDecimal>,
    /// [EChartsBorderHolder]
    pub border: Option<ChartBorder>,
    /// [EChartsFontHolder]
    pub font: Option<ChartFont>,
    /// [EChartsAlignmentHolder]
    pub horizontal_align: Option<EChartsHorizontalAlignment>,
    pub vertical_align: Option<EChartsVerticalAlignment>,
}

#[adapt_model(storable)]
pub struct EChartsYAxis {
    pub show: Option<bool>,
    pub position: Option<EChartsYAxisPosition>,
    pub r#type: Option<EChartsYAxisType>,
    pub name: Option<EChartsYAxisName>,
    pub label: Option<EChartsYAxisLabel>,
    pub auto_min: Option<bool>,
    pub min: Option<BigDecimal>,
    pub auto_max: Option<bool>,
    pub max: Option<BigDecimal>,
    /// [EChartsAxisSplitLineHolder]
    pub split_line: Option<EChartsAxisSplitLine>,
    /// [EChartsAxisMinorSplitLineHolder]
    pub minor_split_line: Option<EChartsAxisSplitLine>,
}

#[adapt_model(storable)]
pub struct EChartsYAxisHolder {
    pub yaxis: Option<EChartsYAxis>,
}

#[derive(Display, Serde, StrEnum)]
pub enum EChartsToolboxOrient {
    Horizontal,
    Vertical,
}

#[adapt_model(storable)]
pub struct EChartsToolbox {
    pub show: Option<bool>,
    pub orient: Option<EChartsToolboxOrient>,
    /// [EChartsPositionHolder]
    pub position: Option<EChartsPosition>,
}

#[adapt_model(storable)]
pub struct EChartsToolboxHolder {
    pub toolbox: Option<EChartsToolbox>,
}

pub type EChartsScriptsVars = HashMap<String, String>;

#[adapt_model(storable)]
pub struct EChartsScriptHolder {
    pub script: Option<String>,
    pub script_vars_defs: Option<String>,
    pub script_vars: Option<EChartsScriptsVars>,
}

#[adapt_model(storable)]
pub struct EChartsSettings {
    /// [EChartsTitleHolder]
    pub title: Option<EChartsTitle>,
    /// [ChartSettings]
    pub border: Option<ChartBorder>,
    pub background_color: Option<ChartColor>,
    pub color_series: Option<PredefinedChartColorSeries>,
    /// [ChartTruncationHolder]
    pub truncation: Option<ChartTruncation>,
}

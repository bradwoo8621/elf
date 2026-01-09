use crate::{
	BarChart, BaseDataModel, CountChart, CustomizedChart, DoughnutChart, EChartsSettings,
	LineChart, MapChart, ModelErrorCode, NightingaleChart, PieChart, ScatterChart, Storable,
	SunburstChart, TreeChart, TreemapChart,
};
use bigdecimal::BigDecimal;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};
use serde::{Deserialize, Serialize};

#[derive(Display, Serde, StrEnum)]
pub enum PredefinedChartColorSeries {
    Regular,
    Dark,
    Light,
}

#[derive(Display, Serde, StrEnum)]
pub enum ChartBorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
}

#[derive(Display, Serde, StrEnum)]
pub enum ChartFontStyle {
    Normal,
    Italic,
}

#[derive(Display, Serde, StrEnum)]
pub enum ChartFontWeight {
    #[display = "100"]
    W100,
    #[display = "200"]
    W200,
    #[display = "300"]
    W300,
    #[display = "400"]
    W400,
    #[display = "500"]
    W500,
    #[display = "600"]
    W600,
    #[display = "700"]
    W700,
    #[display = "800"]
    W800,
    #[display = "900"]
    W900,
}

pub type ChartColor = String;

#[adapt_model(storable)]
pub struct ChartFont {
    pub family: Option<String>,
    pub size: Option<BigDecimal>,
    pub color: Option<ChartColor>,
    pub style: Option<ChartFontStyle>,
    pub weight: Option<ChartFontWeight>,
}

#[adapt_model(storable)]
pub struct ChartBorder {
    pub color: Option<ChartColor>,
    pub style: Option<ChartBorderStyle>,
    pub width: Option<BigDecimal>,
    pub radius: Option<BigDecimal>,
}

#[derive(Display, Serde, StrEnum)]
pub enum ChartTruncationType {
    None,
    Top,
    Bottom,
}

#[adapt_model(storable)]
pub struct ChartTruncation {
    pub r#type: Option<ChartTruncationType>,
    pub count: Option<i32>,
}

#[adapt_model(storable)]
pub struct ChartTruncationHolder {
    pub truncation: Option<ChartTruncation>,
}

#[adapt_model(storable)]
pub struct ChartSettings {
    pub border: Option<ChartBorder>,
    pub background_color: Option<ChartColor>,
    pub color_series: Option<PredefinedChartColorSeries>,
    /// [ChartTruncationHolder]
    pub truncation: Option<ChartTruncation>,
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ChartSettingsRecitation {
    Chart(ChartSettings),
    ECharts(EChartsSettings),
}

#[adapt_model(storable)]
pub struct Chart {
    pub r#type: Option<ChartType>,
    pub settings: Option<ChartSettingsRecitation>,
}

#[derive(Display, Serde, StrEnum)]
pub enum ChartType {
    Count,
    Bar,
    Line,
    Scatter,
    Pie,
    Doughnut,
    Nightingale,
    Sunburst,
    Tree,
    Treemap,
    Map,
    Customized,
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "type")]
pub enum ChartRecitation {
    #[serde(rename = "count")]
    Count(CountChart),
    #[serde(rename = "bar")]
    Bar(BarChart),
    #[serde(rename = "line")]
    Line(LineChart),
    #[serde(rename = "scatter")]
    Scatter(ScatterChart),
    #[serde(rename = "pie")]
    Pie(PieChart),
    #[serde(rename = "doughnut")]
    Doughnut(DoughnutChart),
    #[serde(rename = "nightingale")]
    Nightingale(NightingaleChart),
    #[serde(rename = "sunburst")]
    Sunburst(SunburstChart),
    #[serde(rename = "tree")]
    Tree(TreeChart),
    #[serde(rename = "treemap")]
    Treemap(TreemapChart),
    #[serde(rename = "map")]
    Map(MapChart),
    #[serde(rename = "customized")]
    Customized(CustomizedChart),
}

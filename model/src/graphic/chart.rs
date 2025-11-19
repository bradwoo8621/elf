use crate::graphic::map_chart::MapChart;
use crate::graphic::sunburst_chart::SunburstChart;
use crate::{
    BarChart, BaseDataModel, CountChart, CustomizedChart, DoughnutChart, EChartsSettings,
    LineChart, NightingaleChart, PieChart, ScatterChart, Storable, TreeChart, TreemapChart,
};
use bigdecimal::BigDecimal;
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum PredefinedChartColorSeries {
    Regular,
    Dark,
    Light,
}

#[derive(Display, Serde)]
pub enum ChartBorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
}

#[derive(Display, Serde)]
pub enum ChartFontStyle {
    Normal,
    Italic,
}

#[derive(Display, Serde)]
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

#[derive(Display, Serde)]
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

/// TODO is it workable?
#[adapt_model(storable)]
pub enum ChartSettingsRecitation {
    Chart(ChartSettings),
    ECharts(EChartsSettings),
}

#[adapt_model(storable)]
pub struct Chart {
    pub r#type: Option<ChartType>,
    pub settings: Option<ChartSettingsRecitation>,
}

#[derive(Display, Serde)]
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

// todo is it workable?
#[adapt_model(storable)]
pub enum ChartRecitation {
    Count(CountChart),
    Bar(BarChart),
    Line(LineChart),
    Scatter(ScatterChart),
    Pie(PieChart),
    Doughnut(DoughnutChart),
    Nightingale(NightingaleChart),
    Sunburst(SunburstChart),
    Tree(TreeChart),
    Treemap(TreemapChart),
    Map(MapChart),
    Customized(CustomizedChart),
}

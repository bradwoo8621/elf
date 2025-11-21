use crate::{
    BaseDataModel, ChartBorder, ChartColor, ChartTruncation, ChartType, EChartsGridPositionOnly,
    EChartsTitle, PredefinedChartColorSeries, Storable,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum TreeLayout {
    Orthogonal,
    Radial,
}

#[derive(Display, Serde)]
pub enum TreeOrient {
    #[display = "LR"]
    LeftRight,
    #[display = "RL"]
    RightLeft,
    #[display = "TB"]
    TopBottom,
    #[display = "BT"]
    BottomTop,
}

#[adapt_model(storable)]
pub struct TreeChartSettingsSeries {
    pub layout: Option<TreeLayout>,
    pub orient: Option<TreeOrient>,
    pub roam: Option<bool>,
}

#[adapt_model(storable)]
pub struct TreeChartSettings {
    pub series: Option<TreeChartSettingsSeries>,
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
pub struct TreeChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<TreeChartSettings>,
}

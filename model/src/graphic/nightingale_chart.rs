use crate::{BaseDataModel, ChartType, PieChartSettings, Storable};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct NightingaleChart {
    /// [Chart]
    pub r#type: Option<ChartType>,
    pub settings: Option<PieChartSettings>,
}

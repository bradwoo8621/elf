use crate::{
    BaseDataModel, Pageable, ParameterCondition, Storable, SubjectDatasetColumnId, SubjectId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum SubjectDatasetCriteriaIndicatorArithmetic {
    None,
    #[display = "distinct_count"]
    DistinctCount,
    Count,
    Sum,
    Avg,
    Max,
    Min,
}

#[adapt_model(storable)]
pub struct SubjectDatasetCriteriaIndicator {
    pub name: Option<String>,
    pub column_id: Option<SubjectDatasetColumnId>,
    pub arithmetic: Option<SubjectDatasetCriteriaIndicatorArithmetic>,
    pub alias: Option<String>,
}

#[adapt_model(storable)]
pub struct SubjectDatasetCriteria {
    /// use one of subject id or name
    pub subject_id: Option<SubjectId>,
    pub subject_name: Option<String>,
    pub indicators: Option<Vec<SubjectDatasetCriteriaIndicator>>,
    pub conditions: Option<Vec<ParameterCondition>>,
    pub page_number: Option<i32>,
    pub page_size: Option<i32>,
}

impl Pageable for SubjectDatasetCriteria {
    fn page_number(&self) -> i32 {
        if let Some(page_number) = self.page_number {
            page_number
        } else {
            1
        }
    }

    fn page_size(&self) -> i32 {
        if let Some(page_size) = self.page_size {
            page_size
        } else {
            20
        }
    }
}

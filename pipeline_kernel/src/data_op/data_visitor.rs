use crate::{ArcTopicDataValue, DataPath, DataVisitorBase};
use std::collections::HashMap;
use std::sync::Arc;
use watchmen_model::StdR;

pub trait DataVisitor {
    fn value_of(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitor for HashMap<String, Arc<ArcTopicDataValue>> {
    fn value_of(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        match path {
            DataPath::Simple(parsed_path) => self.value_of_simple_path(parsed_path),
            DataPath::Complex(parsed_path) => self.value_of_complex_path(parsed_path),
        }
    }
}

use crate::{ArcTopicData, ArcTopicDataValue, DataPath, DataVisitorBase};
use elf_base::StdR;
use std::sync::Arc;

pub trait DataVisitor {
    fn value_of(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitor for ArcTopicData {
    fn value_of(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        self.value_of_path(path)
    }
}

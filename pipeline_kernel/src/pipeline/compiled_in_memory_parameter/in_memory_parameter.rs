use crate::{ArcTopicDataValue, PipelineExecutionVariables};
use elf_base::StdR;
use std::sync::Arc;

pub trait InMemoryParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>>;
}

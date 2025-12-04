use crate::PipelineExecutionVariables;
use watchmen_model::TopicDataValue;

pub trait InMemoryParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> &TopicDataValue;
}

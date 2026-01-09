use crate::PipelineExecutionVariables;
use elf_base::StdR;

pub trait InMemoryParameterCondition {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool>;
    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool>;
}

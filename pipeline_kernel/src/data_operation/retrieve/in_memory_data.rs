use crate::{
    ArcTopicDataValue, DataPath, DataPathSegment, FuncDataPath, PipelineExecutionVariables,
    PipelineKernelErrorCode, PlainDataPath,
};
use elf_base::{ErrorCode, StdR};
use std::sync::Arc;

pub struct InMemoryData<'a> {
    variables: &'a PipelineExecutionVariables,
    current_data_only: bool,
}

impl<'a> InMemoryData<'a> {
    pub fn new(variables: &'a PipelineExecutionVariables) -> Self {
        Self {
            variables,
            current_data_only: false,
        }
    }

    /// get an instance which allowed get value from current data only
    pub fn current_only(&mut self) -> &Self {
        self.current_data_only = true;
        self
    }

    /// get an instance which allowed get value from anywhere (current, previous and variables)
    pub fn all_allowed(&mut self) -> &Self {
        self.current_data_only = false;
        self
    }
}

impl InMemoryData<'_> {
    fn get_first_part_by_plain_path(&self, path: &PlainDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement get_first_part_by_plain_path for InMemoryData")
    }

    fn get_first_part_by_func_path(&self, path: &FuncDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement get_first_part_by_func_path for InMemoryData")
    }

    fn get_first_part(&self, segment: &DataPathSegment) -> StdR<Arc<ArcTopicDataValue>> {
        match segment {
            DataPathSegment::Plain(plain_path) => self.get_first_part_by_plain_path(plain_path),
            DataPathSegment::Func(func_path) => self.get_first_part_by_func_path(func_path),
        }
    }

    fn get_rest_part(
        &self,
        parent: Arc<ArcTopicDataValue>,
        segment: &DataPathSegment,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement get_rest_part for InMemoryData")
    }

    pub fn get_value(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let segments = path.segments();
        if let Some((first, rest)) = segments.split_first() {
            let top_value = self.get_first_part(first)?;
            if rest.is_empty() {
                Ok(top_value)
            } else {
                let mut last_value = top_value;
                for segment in rest {
                    let value = self.get_rest_part(last_value, segment)?;
                    last_value = value;
                }

                Ok(last_value)
            }
        } else {
            PipelineKernelErrorCode::IncorrectDataPath.msg("Data path is empty.")
        }
    }
}

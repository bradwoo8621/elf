use crate::PipelineKernelErrorCode;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerData, PipelineTriggerTraceId, StdErr, StdErrorCode, StringUtils, TopicDataId,
};

pub struct PipelineEntrypoint {
    principal: Principal,
    trace_id: Option<PipelineTriggerTraceId>,
}

impl PipelineEntrypoint {
    pub fn with(principal: Principal) -> Self {
        PipelineEntrypoint {
            principal,
            trace_id: None,
        }
    }

    pub fn traced_with(mut self, trace_id: PipelineTriggerTraceId) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    fn check_trigger_code(trigger_data: &PipelineTriggerData) -> Result<(), StdErr> {
        if let Some(code) = &trigger_data.code {
            if code.is_blank() {
                StdErr::of(
                    PipelineKernelErrorCode::TriggerCodeIsBlank.code(),
                    "Pipeline trigger code cannot be blank.",
                )
            } else {
                Ok(())
            }
        } else {
            StdErr::of(
                PipelineKernelErrorCode::TriggerCodeMissed.code(),
                "Pipeline trigger code cannot be empty.",
            )
        }
    }

    fn check_trigger_type(trigger_data: &PipelineTriggerData) -> Result<(), StdErr> {
        if trigger_data.trigger_type.is_none() {
            StdErr::of(
                PipelineKernelErrorCode::TriggerTypeMissed.code(),
                "Pipeline trigger type cannot be empty.",
            )
        } else {
            Ok(())
        }
    }

    fn check_trigger_data(trigger_data: &PipelineTriggerData) -> Result<(), StdErr> {
        if trigger_data.data.is_none() {
            StdErr::of(
                PipelineKernelErrorCode::TriggerDataMissed.code(),
                "Pipeline trigger data cannot be empty.",
            )
        } else {
            Ok(())
        }
    }

    fn check(trigger_data: &PipelineTriggerData) -> Result<(), StdErr> {
        let mut errors = Vec::new();
        vec![
            PipelineEntrypoint::check_trigger_code,
            PipelineEntrypoint::check_trigger_type,
            PipelineEntrypoint::check_trigger_data,
        ]
        .iter()
        .for_each(|f| {
            if let Err(e) = f(trigger_data) {
                errors.push(e);
            }
        });

        match errors.len() {
            0 => Ok(()),
            1 => Err(errors.remove(0)),
            _ => StdErr::me(errors),
        }
    }

    pub fn execute(&self, trigger_data: PipelineTriggerData) -> Result<TopicDataId, StdErr> {
        PipelineEntrypoint::check(&trigger_data)?;

        todo!("implement execute for PipelineEntrypoint")
    }

    pub async fn execute_async(
        &self,
        trigger_data: PipelineTriggerData,
    ) -> Result<TopicDataId, StdErr> {
        PipelineEntrypoint::check(&trigger_data)?;

        todo!("implement execute_async for PipelineEntrypoint")
    }
}

#[cfg(test)]
mod tests {
    use crate::PipelineEntrypoint;
    use watchmen_auth::Principal;
    use watchmen_model::{PipelineTriggerData, PipelineTriggerType};

    #[test]
    fn test() {
        let trigger_data = PipelineTriggerData::new()
            .code(String::from("topic-1"))
            .trigger_type(PipelineTriggerType::Insert)
            .tenant_id(String::from("tenant-1"));
        let result = PipelineEntrypoint::with(Principal::fake_super_admin())
            .traced_with("".to_string())
            .execute(trigger_data);
        assert!(result.is_ok());
    }
}

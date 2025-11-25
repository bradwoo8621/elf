use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerData, PipelineTriggerTraceId, PipelineTriggerType, StdErr, TopicDataId,
};
use watchmen_runtime_model_kernel::TopicSchema;

type TopicDataService = String;
type TopicTrigger = String;

pub struct PipelineTrigger {
    // storages: RuntimeTopicStorages,
    trigger_topic_schema: TopicSchema,
    trigger_type: PipelineTriggerType,
    trigger_data: PipelineTriggerData,
    trace_id: PipelineTriggerTraceId,
    principal: Principal,
    asynchronized: bool,
    // handle_monitor_log: handle_monitor_log,
}

impl PipelineTrigger {
    // pub fn new() -> Self {
    // 	PipelineTrigger {}
    // }

    fn prepare_trigger_data(&self) {
        // self.trigger_topic_schema.prepare_data(self.trigger_data, self.principal)
    }

    fn ask_topic_data_service(&self, schema: TopicSchema) -> TopicDataService {
        // self.storages
        // 	.ask_topic_storage(schema)
        // 	.ask_topic_data_service(schema, self.principal)
        todo!("Not implemented yet")
    }

    fn save_trigger_data(&self) -> Result<TopicTrigger, StdErr> {
        // match self.trigger_topic_schema.get_topic().kind {
        // 	Some(TopicKind::Synonym) => {
        // 		// only insertion is supported on synonym
        // 		// will do nothing on synonym topic itself, simply trigger the insert pipeline
        // 		// typically, there should a historical topic to handle data from synonym topic
        // 		// and process data based on historical topic insertion
        // 		match self.trigger_type {
        // 			PipelineTriggerType::Insert => {
        // 				Ok(TopicTrigger::new(
        // 					previous: None,
        // 					current: self.trigger_data,
        // 					trigger_type: PipelineTriggerType::Insert,
        // 					internal_data_id: String::from("-1"),
        // 				))
        // 			}
        // 			_ => StdErr::of(
        // 				PipelineKernelErrorCode::TriggerTypeNotSupportedOnSynonym.code(),
        // 				format!(
        // 					"Trigger type[{}] is not supported on synonym.",
        // 					self.trigger_type
        // 				),
        // 			),
        // 		}
        // 	}
        // 	_ => {
        // 		let data_service = self.ask_topic_data_service(self.trigger_topic_schema);
        // 		match self.trigger_type {
        // 			PipelineTriggerType::Insert => data_service.trigger_by_insert(self.trigger_data),
        // 			PipelineTriggerType::InsertOrMerge => {
        // 				data_service.trigger_by_insert_or_merge(self.trigger_data)
        // 			}
        // 			PipelineTriggerType::Merge => data_service.trigger_by_merge(self.trigger_data),
        // 			PipelineTriggerType::Delete => data_service.trigger_by_delete(self.trigger_data),
        // 		}
        // 	}
        // }
        todo!("Not implemented yet")
    }

    /// 1. prepare trigger data
    /// 2. save trigger data
    /// 3. start pipeline execution
    pub async fn invoke(&self) -> Result<TopicDataId, StdErr> {
        self.prepare_trigger_data();
        let result = self.save_trigger_data()?;
        if self.asynchronized {
            // TODO ensure_future(self.start(result))
        } else {
            // TODO await self.start(result)
        }
        // result.internal_data_id
        todo!("Not implemented yet");
    }
}

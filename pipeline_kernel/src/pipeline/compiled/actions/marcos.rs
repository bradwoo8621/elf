#[macro_export]
macro_rules! generate_compiled_action {
    ($struct_name:ident {
        $($field:ident : $field_type:ty),* $(,)?
    }) => {
        paste::paste! {
            pub struct [<Compiled $struct_name Action>] {
                pipeline: std::sync::Arc<ArcPipeline>,
                stage: std::sync::Arc<ArcPipelineStage>,
                unit: std::sync::Arc<ArcPipelineUnit>,
                action: std::sync::Arc<[<Arc $struct_name Action>]>,

                $($field : $field_type),*
            }

            impl [<Compiled $struct_name Action>] {
                pub fn pipeline(&self) -> &Arc<ArcPipeline> {
                    &self.pipeline
                }

                pub fn stage(&self) -> &Arc<ArcPipelineStage> {
                    &self.stage
                }

                pub fn unit(&self) -> &Arc<ArcPipelineUnit> {
                    &self.unit
                }

                pub fn action(&self) -> &Arc<[<Arc $struct_name Action>]> {
                    &self.action
                }
            }
        }
    };
}

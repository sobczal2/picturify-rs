use std::sync::{Arc, RwLock};
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::negative::NegativeProcessor;
use crate::common::pipeline_progress::PipelineProgress;

pub struct NegativePipelineOptions {}

pub struct NegativePipeline {
    options: NegativePipelineOptions,
}

impl NegativePipeline {
    pub fn new(_negative_pipeline_options: NegativePipelineOptions) -> Self {
        Self {
            options: NegativePipelineOptions {},
        }
    }
}

impl Pipeline for NegativePipeline {
    fn run(&self, fast_image: FastImage, pipeline_progress: Arc<RwLock<PipelineProgress>>) -> FastImage {
        pipeline_progress.write().unwrap().setup_combined(1);
        pipeline_progress.write().unwrap().new_individual("Negative".to_string());

        let processor = NegativeProcessor::new();
        let final_image = processor.process(fast_image, pipeline_progress.read().unwrap().get_individual_progress("Negative".to_string()));
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}

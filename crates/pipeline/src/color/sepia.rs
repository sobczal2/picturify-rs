use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;
use std::sync::{Arc, RwLock};

pub struct SepiaPipelineOptions {}

pub struct SepiaPipeline {
    #[allow(dead_code)]
    options: SepiaPipelineOptions,
}

impl SepiaPipeline {
    pub fn new(options: SepiaPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for SepiaPipeline {
    fn run(
        &self,
        fast_image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });
        
        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.setup_combined(1);
        pipeline_progress_write.new_individual("Sepia".to_string());
        drop(pipeline_progress_write);

        let processor = SepiaProcessor::new();
        let final_image = processor.process(
            fast_image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}

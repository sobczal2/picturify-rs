use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::noise::mean::{MeanProcessor, MeanProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct MeanPipelineOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

pub struct MeanPipeline {
    options: MeanPipelineOptions,
}

impl MeanPipeline {
    pub fn new(options: MeanPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for MeanPipeline {
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
        pipeline_progress_write.new_individual("Mean".to_string());
        drop(pipeline_progress_write);

        let processor = MeanProcessor::new().with_options(MeanProcessorOptions {
            radius: self.options.radius,
            use_fast_approximation: self.options.use_fast_approximation,
        });
        let final_image = processor.process(
            fast_image,
            pipeline_progress
                .read()
                .unwrap()
                .get_individual_progress("Mean".to_string()),
        );
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}
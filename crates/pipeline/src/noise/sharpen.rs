use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::noise::sharpen::{SharpenProcessor, SharpenProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct SharpenPipelineOptions {
    pub use_fast_approximation: bool,
}

pub struct SharpenPipeline {
    options: SharpenPipelineOptions,
}

impl SharpenPipeline {
    pub fn new(options: SharpenPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for SharpenPipeline {
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
        pipeline_progress_write.new_individual("Sharpen".to_string());
        drop(pipeline_progress_write);

        let processor = SharpenProcessor::new().with_options(SharpenProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
        });
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
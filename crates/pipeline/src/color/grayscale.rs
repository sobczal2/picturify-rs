use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::grayscale::{GrayscaleProcessor, GrayscaleProcessorOptions, GrayscaleStrategy};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct GrayscalePipelineOptions {
    pub strategy: GrayscaleStrategy,
    pub use_fast_approximation: bool,
}

pub struct GrayscalePipeline {
    options: GrayscalePipelineOptions,
}

impl GrayscalePipeline {
    pub fn new(options: GrayscalePipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for GrayscalePipeline {
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
        pipeline_progress_write.new_individual("Grayscale".to_string());
        drop(pipeline_progress_write);

        let processor = GrayscaleProcessor::new().with_options(GrayscaleProcessorOptions {
            strategy: self.options.strategy,
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
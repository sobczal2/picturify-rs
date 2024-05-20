use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::grayscale::{
    GrayscaleProcessor, GrayscaleProcessorOptions, GrayscaleStrategy,
};
use std::sync::{Arc, RwLock};

pub struct GrayscalePipelineOptions {
    pub strategy: GrayscaleStrategy,
    pub fast: bool,
}

impl Default for GrayscalePipelineOptions {
    fn default() -> Self {
        GrayscalePipelineOptions {
            strategy: GrayscaleStrategy::Average,
            fast: false,
        }
    }
}

pub struct GrayscalePipeline {
    options: GrayscalePipelineOptions,
}

impl GrayscalePipeline {
    pub fn new(options: GrayscalePipelineOptions) -> Self {
        Self { options }
    }
}

const GRAYSCALE_PROCESSOR_NAME: &str = "Grayscale";

impl Pipeline for GrayscalePipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress =
            pipeline_progress.unwrap_or_else(|| Arc::new(RwLock::new(PipelineProgress::new())));

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(GRAYSCALE_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(1);
        drop(pipeline_progress_write);

        let processor = GrayscaleProcessor::new().with_options(GrayscaleProcessorOptions {
            strategy: self.options.strategy,
            use_fast_approximation: self.options.fast,
        });
        let final_image = processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}

use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::grayscale::{
    GrayscaleProcessor, GrayscaleProcessorOptions, GrayscaleStrategy,
};

pub struct GrayscalePipelineOptions {
    pub strategy: GrayscaleStrategy,
    pub fast: bool,
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
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(GRAYSCALE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = GrayscaleProcessor::new(GrayscaleProcessorOptions {
            strategy: self.options.strategy,
            use_fast_approximation: self.options.fast,
        });
        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();
        final_image
    }
}

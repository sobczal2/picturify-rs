use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::{SepiaProcessor, SepiaProcessorOptions};

pub struct SepiaPipelineOptions {
    pub fast: bool,
}

pub struct SepiaPipeline {
    options: SepiaPipelineOptions,
}

impl SepiaPipeline {
    pub fn new(options: SepiaPipelineOptions) -> Self {
        Self { options }
    }
}

const SEPIA_PROCESSOR_NAME: &str = "Sepia";

impl Pipeline for SepiaPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(SEPIA_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = SepiaProcessor::new(SepiaProcessorOptions {
            use_fast_approximation: self.options.fast,
        });
        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();
        final_image
    }
}

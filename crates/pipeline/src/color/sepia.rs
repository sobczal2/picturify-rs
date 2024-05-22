use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;

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

const SEPIA_PROCESSOR_NAME: &str = "Sepia";

impl Pipeline for SepiaPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(SEPIA_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = SepiaProcessor::new();
        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();
        final_image
    }
}

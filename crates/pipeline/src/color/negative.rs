use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::negative::{NegativeProcessor, NegativeProcessorOptions};

pub struct NegativePipelineOptions {
    pub fast: bool,
}

pub struct NegativePipeline {
    #[allow(dead_code)]
    options: NegativePipelineOptions,
}

impl NegativePipeline {
    pub fn new(options: NegativePipelineOptions) -> Self {
        Self { options }
    }
}

const NEGATIVE_PROCESSOR_NAME: &str = "Negative";

impl Pipeline for NegativePipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(NEGATIVE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = NegativeProcessor::new().with_options(NegativeProcessorOptions {
            use_fast_approximation: self.options.fast,
        });
        
        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();
        final_image
    }
}

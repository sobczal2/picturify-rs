use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::brightness::{
    BrightnessProcessor, BrightnessProcessorOptions,
};

pub struct BrightnessPipelineOptions {
    pub factor: f32,
}

pub struct BrightnessPipeline {
    options: BrightnessPipelineOptions,
}

impl BrightnessPipeline {
    pub fn new(options: BrightnessPipelineOptions) -> Self {
        Self { options }
    }
}

const BRIGHTNESS_PROCESSOR_NAME: &str = "Brightness";

impl Pipeline for BrightnessPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(BRIGHTNESS_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = BrightnessProcessor::new(BrightnessProcessorOptions {
            factor: self.options.factor,
        });
        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());

        pipeline_progress.increment_combined();
        final_image
    }
}

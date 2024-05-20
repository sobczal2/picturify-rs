use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::brightness::{BrightnessProcessor, BrightnessProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct BrightnessPipelineOptions {
    pub factor: f32,
}

impl Default for BrightnessPipelineOptions {
    fn default() -> Self {
        BrightnessPipelineOptions { factor: 1.0 }
    }
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
    fn run(&self, image: FastImage, pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| Arc::new(RwLock::new(PipelineProgress::new())));

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(BRIGHTNESS_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(1);
        drop(pipeline_progress_write);

        let processor = BrightnessProcessor::new().with_options(BrightnessProcessorOptions {
            factor: self.options.factor,
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
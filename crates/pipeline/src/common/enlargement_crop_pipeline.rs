use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::geometry::crop::{CropProcessor, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementProcessor, EnlargementProcessorOptions,
};

use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct EnlargementCropPipelineOptions {
    pub fast: bool,
    pub processor_name: String,
    pub processor: Box<dyn Processor>,
    pub enlargement_processor_options: EnlargementProcessorOptions,
    pub crop_processor_options: CropProcessorOptions,
}

pub struct EnlargementCropPipeline {
    options: EnlargementCropPipelineOptions,
}

impl EnlargementCropPipeline {
    pub fn new(options: EnlargementCropPipelineOptions) -> Self {
        Self { options }
    }
}

const ENLARGEMENT_PROCESSOR_NAME: &str = "Enlargement";
const CROP_PROCESSOR_NAME: &str = "Crop";

impl Pipeline for EnlargementCropPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        match self.options.fast {
            true => self.run_fast(image, pipeline_progress),
            false => self.run_slow(image, pipeline_progress),
        }
    }
}

impl EnlargementCropPipeline {
    fn run_fast(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(self.options.processor_name.clone());
        pipeline_progress.setup_combined(1);

        let image = self
            .options
            .processor
            .process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();

        image
    }

    fn run_slow(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(ENLARGEMENT_PROCESSOR_NAME.to_string());
        pipeline_progress.new_individual(self.options.processor_name.clone());
        pipeline_progress.new_individual(CROP_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(3);

        let enlargement_processor =
            EnlargementProcessor::new().with_options(self.options.enlargement_processor_options);
        let image = enlargement_processor
            .process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();

        let image = self
            .options
            .processor
            .process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();

        let crop_processor = CropProcessor::new().with_options(self.options.crop_processor_options);

        let image =
            crop_processor.process(image, pipeline_progress.get_current_individual_progress());
        pipeline_progress.increment_combined();
        image
    }
}

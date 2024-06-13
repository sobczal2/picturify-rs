use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::geometry::crop::{
    CropBorder, CropProcessor, CropProcessorOptions,
};

#[derive(Copy, Clone)]
pub struct CropPipelineOptions {
    pub crop_border: CropBorder,
}

pub struct CropPipeline {
    options: CropPipelineOptions,
}

impl CropPipeline {
    pub fn new(options: CropPipelineOptions) -> Self {
        Self { options }
    }
}

const CROP_PROCESSOR_NAME: &str = "Crop";

impl Pipeline for CropPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(CROP_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = CropProcessor::new(CropProcessorOptions {
            crop_border: self.options.crop_border,
        });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());

        pipeline_progress.increment_combined();
        final_image
    }
}

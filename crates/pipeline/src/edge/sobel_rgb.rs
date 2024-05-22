use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::WithOptions;
use picturify_processing::processors::edge::sobel_rgb::{
    SobelRgbProcessor, SobelRgbProcessorOptions,
};
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct SobelRgbPipelineOptions {
    pub fast: bool,
}

pub struct SobelRgbPipeline {
    options: SobelRgbPipelineOptions,
}

impl SobelRgbPipeline {
    pub fn new(options: SobelRgbPipelineOptions) -> Self {
        Self { options }
    }
}

const SOBEL_RGB_PROCESSOR_NAME: &str = "SobelRgb";

impl Pipeline for SobelRgbPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let processor = SobelRgbProcessor::new().with_options(SobelRgbProcessorOptions {
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: SOBEL_RGB_PROCESSOR_NAME.to_string(),
            processor: Box::new(processor),
            enlargement_processor_options: EnlargementProcessorOptions {
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
                border: EnlargementBorder::from_all(1),
            },
            crop_processor_options: CropProcessorOptions {
                crop_border: CropBorder::new(width, height, 1, 1),
            },
        });

        pipeline.run(image, pipeline_progress)
    }
}

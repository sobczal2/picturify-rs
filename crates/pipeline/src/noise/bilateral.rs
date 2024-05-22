use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::WithOptions;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::bilateral::{
    BilateralProcessor, BilateralProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct BilateralPipelineOptions {
    pub radius: usize,
    pub sigma_spatial: f32,
    pub sigma_intensity: f32,
    pub fast: bool,
}

pub struct BilateralPipeline {
    options: BilateralPipelineOptions,
}

impl BilateralPipeline {
    pub fn new(options: BilateralPipelineOptions) -> Self {
        Self { options }
    }
}

const BILATERAL_PROCESSOR_NAME: &str = "Bilateral";

impl Pipeline for BilateralPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let processor = BilateralProcessor::new().with_options(BilateralProcessorOptions {
            radius: self.options.radius,
            sigma_spatial: self.options.sigma_spatial,
            sigma_intensity: self.options.sigma_intensity,
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: BILATERAL_PROCESSOR_NAME.to_string(),
            processor: Box::new(processor),
            enlargement_processor_options: EnlargementProcessorOptions {
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
                border: EnlargementBorder::from_all(self.options.radius),
            },
            crop_processor_options: CropProcessorOptions {
                crop_border: CropBorder::new(
                    width,
                    height,
                    self.options.radius,
                    self.options.radius,
                ),
            },
        });

        pipeline.run(image, pipeline_progress)
    }
}

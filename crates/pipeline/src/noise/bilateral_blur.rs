use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::bilateral_blur::{
    BilateralBlurProcessor, BilateralBlurProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct BilateralBlurPipelineOptions {
    pub radius: usize,
    pub sigma_spatial: f32,
    pub sigma_intensity: f32,
    pub fast: bool,
}

pub struct BilateralBlurPipeline {
    options: BilateralBlurPipelineOptions,
}

impl BilateralBlurPipeline {
    pub fn new(options: BilateralBlurPipelineOptions) -> Self {
        Self { options }
    }
}

const BILATERAL_BLUR_PROCESSOR_NAME: &str = "BilateralBlur";

impl Pipeline for BilateralBlurPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let processor = BilateralBlurProcessor::new(BilateralBlurProcessorOptions {
            radius: self.options.radius,
            sigma_spatial: self.options.sigma_spatial,
            sigma_intensity: self.options.sigma_intensity,
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: BILATERAL_BLUR_PROCESSOR_NAME.to_string(),
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

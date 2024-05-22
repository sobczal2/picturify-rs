use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::WithOptions;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::gaussian_blur::{
    GaussianBlurProcessor, GaussianBlurProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct GaussianBlurPipelineOptions {
    pub radius: usize,
    pub sigma: f32,
    pub fast: bool,
}

pub struct GaussianBlurPipeline {
    options: GaussianBlurPipelineOptions,
}

impl GaussianBlurPipeline {
    pub fn new(options: GaussianBlurPipelineOptions) -> Self {
        Self { options }
    }
}

const GAUSSIAN_BLUR_PROCESSOR_NAME: &str = "GaussianBlur";

impl Pipeline for GaussianBlurPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let processor = GaussianBlurProcessor::new().with_options(GaussianBlurProcessorOptions {
            radius: self.options.radius,
            sigma: self.options.sigma,
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: GAUSSIAN_BLUR_PROCESSOR_NAME.to_string(),
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

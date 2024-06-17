use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::mean_blur::{
    MeanBlurProcessor, MeanBlurProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct MeanBlurPipelineOptions {
    pub radius: usize,
    pub fast: bool,
}

pub struct MeanBlurPipeline {
    options: MeanBlurPipelineOptions,
}

impl MeanBlurPipeline {
    pub fn new(options: MeanBlurPipelineOptions) -> Self {
        Self { options }
    }
}

const MEAN_BLUR_PROCESSOR_NAME: &str = "MeanBlur";

impl Pipeline for MeanBlurPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let processor = MeanBlurProcessor::new(MeanBlurProcessorOptions {
            radius: self.options.radius,
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: MEAN_BLUR_PROCESSOR_NAME.to_string(),
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

use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::median_blur::{
    MedianBlurProcessor, MedianBlurProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct MedianBlurPipelineOptions {
    pub radius: usize,
    pub fast: bool,
}

pub struct MedianBlurPipeline {
    options: MedianBlurPipelineOptions,
}

impl MedianBlurPipeline {
    pub fn new(options: MedianBlurPipelineOptions) -> Self {
        Self { options }
    }
}

const MEDIAN_BLUR_PROCESSOR_NAME: &str = "MedianBlur";

impl Pipeline for MedianBlurPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let processor = MedianBlurProcessor::new(MedianBlurProcessorOptions {
            radius: self.options.radius,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: MEDIAN_BLUR_PROCESSOR_NAME.to_string(),
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

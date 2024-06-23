use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::processors::edge::emboss::{EmbossProcessor, EmbossProcessorOptions};
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};

pub struct EmbossPipelineOptions {
    pub fast: bool,
}

pub struct EmbossPipeline {
    options: EmbossPipelineOptions,
}

impl EmbossPipeline {
    pub fn new(options: EmbossPipelineOptions) -> Self {
        Self { options }
    }
}

const EMBOSS_PROCESSOR_NAME: &str = "Emboss";

impl Pipeline for EmbossPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
    ) -> PipelinePicturifyResult<FastImage> {
        let processor = EmbossProcessor::new(EmbossProcessorOptions {
            use_fast_approximation: self.options.fast,
        });

        let (width, height) = image.size().into();

        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: EMBOSS_PROCESSOR_NAME.to_string(),
            processor: Box::new(processor),
            enlargement_processor_options: EnlargementProcessorOptions {
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0).into()),
                border: EnlargementBorder::from_all(1),
            },
            crop_processor_options: CropProcessorOptions {
                crop_border: CropBorder::new(width, height, 1, 1),
            },
        });

        pipeline.run(image, pipeline_progress)
    }
}

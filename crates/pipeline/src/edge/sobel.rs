use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::edge::sobel::{SobelProcessor, SobelProcessorOptions};
use picturify_processing::processors::edge::sobel_rgb::{SobelRgbProcessor, SobelRgbProcessorOptions};
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct SobelPipelineOptions {
    pub fast: bool,
    pub rgb: bool,
}

pub struct SobelPipeline {
    options: SobelPipelineOptions,
}

impl SobelPipeline {
    pub fn new(options: SobelPipelineOptions) -> Self {
        Self { options }
    }
}

const SOBEL_PROCESSOR_NAME: &str = "Sobel";

impl Pipeline for SobelPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        let processor: Box<dyn Processor> = match self.options.rgb {
            true => Box::new(SobelRgbProcessor::new().with_options(SobelRgbProcessorOptions {
                use_fast_approximation: self.options.fast,
            })),
            false => Box::new(SobelProcessor::new().with_options(SobelProcessorOptions {
                use_fast_approximation: self.options.fast,
            })),
        };
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: SOBEL_PROCESSOR_NAME.to_string(),
            processor,
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

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::common::processors::CpuProcessor;
use picturify_processing::processors::edge::prewitt::{PrewittProcessor, PrewittProcessorOptions};
use picturify_processing::processors::edge::prewitt_rgb::{
    PrewittRgbProcessor, PrewittRgbProcessorOptions,
};
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};

pub struct PrewittPipelineOptions {
    pub fast: bool,
    pub rgb: bool,
}

pub struct PrewittPipeline {
    options: PrewittPipelineOptions,
}

impl PrewittPipeline {
    pub fn new(options: PrewittPipelineOptions) -> Self {
        Self { options }
    }
}

const PREWITT_PROCESSOR_NAME: &str = "Prewitt";

impl Pipeline for PrewittPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
    ) -> PipelinePicturifyResult<FastImage> {
        let processor: Box<dyn CpuProcessor> = match self.options.rgb {
            true => Box::new(PrewittRgbProcessor::new(PrewittRgbProcessorOptions {
                use_fast_approximation: self.options.fast,
            })),
            false => Box::new(PrewittProcessor::new(PrewittProcessorOptions {
                use_fast_approximation: self.options.fast,
            })),
        };
        let (width, height) = image.size().into();

        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: PREWITT_PROCESSOR_NAME.to_string(),
            processor,
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

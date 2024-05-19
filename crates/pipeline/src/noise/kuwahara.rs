use std::sync::{Arc, RwLock};

use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::WithOptions;
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::kuwahara::{
    KuwaharaProcessor, KuwaharaProcessorOptions,
};

use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct KuwaharaPipelineOptions {
    pub fast: bool,
    pub radius: usize,
}

pub struct KuwaharaPipeline {
    options: KuwaharaPipelineOptions,
}

impl KuwaharaPipeline {
    pub fn new(options: KuwaharaPipelineOptions) -> Self {
        Self { options }
    }
}

const KUWAHARA_PROCESSOR_NAME: &str = "Kuwahara";

impl Pipeline for KuwaharaPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let processor = KuwaharaProcessor::new().with_options(KuwaharaProcessorOptions {
            radius: self.options.radius,
        });
        let width = image.get_width();
        let height = image.get_height();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: KUWAHARA_PROCESSOR_NAME.to_string(),
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

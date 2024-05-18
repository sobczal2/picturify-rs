use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::noise::kuwahara::{
    KuwaharaProcessor, KuwaharaProcessorOptions,
};
use std::sync::{Arc, RwLock};
use picturify_core::palette::Srgba;
use picturify_processing::processors::geometry::crop::{CropProcessor, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{EnlargementBorder, EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy};

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

impl Pipeline for KuwaharaPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        match self.options.fast {
            true => self.run_fast(image, pipeline_progress),
            false => self.run_slow(image, pipeline_progress),
        }
    }
}

const ENLARGEMENT_PROCESSOR_NAME: &str = "Enlargement";
const KUWAHARA_PROCESSOR_NAME: &str = "Kuwahara";
const CROP_PROCESSOR_NAME: &str = "Crop";

impl KuwaharaPipeline {
    fn run_fast(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(KUWAHARA_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(1);
        drop(pipeline_progress_write);

        let processor = KuwaharaProcessor::new().with_options(KuwaharaProcessorOptions {
            radius: self.options.radius,
        });
        let image = processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();
        image
    }

    fn run_slow(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(ENLARGEMENT_PROCESSOR_NAME.to_string());
        pipeline_progress_write.new_individual(KUWAHARA_PROCESSOR_NAME.to_string());
        pipeline_progress_write.new_individual(CROP_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(3);
        drop(pipeline_progress_write);

        let enlargement_processor = EnlargementProcessor::new().with_options(EnlargementProcessorOptions {
            border: EnlargementBorder::from_all(self.options.radius),
            strategy: EnlargementStrategy::Constant(Srgba::new(0f32, 0f32, 0f32, 0f32)),
        });
        let image = enlargement_processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();

        let kuwahara_processor = KuwaharaProcessor::new().with_options(KuwaharaProcessorOptions {
            radius: self.options.radius,
        });
        let image = kuwahara_processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();

        let crop_processor = CropProcessor::new().with_options(CropProcessorOptions {
            x: self.options.radius,
            y: self.options.radius,
            width: image.get_width() - 2 * self.options.radius,
            height: image.get_height() - 2 * self.options.radius,
        });
        let image = crop_processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();
        image
    }
}
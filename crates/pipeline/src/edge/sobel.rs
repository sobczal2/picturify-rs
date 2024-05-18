use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::edge::sobel::{SobelProcessor, SobelProcessorOptions};
use std::sync::{Arc, RwLock};
use picturify_core::palette::Srgba;
use picturify_processing::processors::geometry::crop::{CropProcessor, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy};

pub struct SobelPipelineOptions {
    pub use_fast_approximation: bool,
}

pub struct SobelPipeline {
    options: SobelPipelineOptions,
}

impl SobelPipeline {
    pub fn new(options: SobelPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for SobelPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        match self.options.use_fast_approximation {
            true => self.run_fast(image, pipeline_progress),
            false => self.run_slow(image, pipeline_progress),
        }
    }
}

const ENLARGEMENT_PROCESSOR_NAME: &str = "Enlargement";
const SOBEL_PROCESSOR_NAME: &str = "Sobel";
const CROP_PROCESSOR_NAME: &str = "Crop";

impl SobelPipeline {
    fn run_fast(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.setup_combined(1);
        pipeline_progress_write.new_individual(SOBEL_PROCESSOR_NAME.to_string());
        drop(pipeline_progress_write);

        let sobel_processor = SobelProcessor::new().with_options(SobelProcessorOptions {
            use_fast_approximation: true,
        });
        let image = sobel_processor.process(
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
        pipeline_progress_write.new_individual(SOBEL_PROCESSOR_NAME.to_string());
        pipeline_progress_write.new_individual(CROP_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(3);
        drop(pipeline_progress_write);

        let enlargement_processor = EnlargementProcessor::new().with_options(EnlargementProcessorOptions {
            border: 1,
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

        let sobel_processor = SobelProcessor::new().with_options(SobelProcessorOptions {
            use_fast_approximation: false,
        });
        let image = sobel_processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();

        let crop_processor = CropProcessor::new().with_options(CropProcessorOptions {
            x: 1,
            y: 1,
            width: image.get_width() - 2,
            height: image.get_height() - 2,
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
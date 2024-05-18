use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::geometry::crop::{CropProcessor, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{EnlargementBorder, EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy};
use picturify_processing::processors::noise::mean::{MeanProcessor, MeanProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct MeanPipelineOptions {
    pub radius: usize,
    pub fast: bool,
}

pub struct MeanPipeline {
    options: MeanPipelineOptions,
}

impl MeanPipeline {
    pub fn new(options: MeanPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for MeanPipeline {
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
const MEAN_PROCESSOR_NAME: &str = "Mean";
const CROP_PROCESSOR_NAME: &str = "Crop";

impl MeanPipeline {
    fn run_fast(
        &self,
        image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(MEAN_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(1);
        drop(pipeline_progress_write);

        let processor = MeanProcessor::new().with_options(MeanProcessorOptions {
            radius: self.options.radius,
            use_fast_approximation: true,
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
        pipeline_progress_write.new_individual(MEAN_PROCESSOR_NAME.to_string());
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

        let mean_processor = MeanProcessor::new().with_options(MeanProcessorOptions {
            use_fast_approximation: false,
            radius: self.options.radius,
        });
        let image = mean_processor.process(
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
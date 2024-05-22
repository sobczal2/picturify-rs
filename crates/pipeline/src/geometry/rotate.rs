use std::sync::{Arc, RwLock};
use picturify_core::common::angle::Angle;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::geometry::rotate_fixed::{RotateFixedProcessor, RotateFixedStrategy, RoteteFixedProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

#[derive(Copy, Clone)]
pub struct RotatePipelineOptions {
    pub angle: Angle,
}

pub struct RotatePipeline {
    options: RotatePipelineOptions,
}

impl RotatePipeline {
    pub fn new(options: RotatePipelineOptions) -> Self {
        Self { options }
    }
}

const ROTATE_PROCESSOR_NAME: &str = "Rotate";

impl Pipeline for RotatePipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>) -> FastImage {
        match RotateFixedStrategy::try_from(self.options.angle) {
            Ok(strategy) => self.run_fixed(image, pipeline_progress, strategy),
            Err(_) => panic!("Invalid angle")
        }
    }
}

impl RotatePipeline {
    fn run_fixed(&self, image: FastImage, pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>, strategy: RotateFixedStrategy) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| Arc::new(RwLock::new(PipelineProgress::new())));

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.new_individual(ROTATE_PROCESSOR_NAME.to_string());
        pipeline_progress_write.setup_combined(1);
        drop(pipeline_progress_write);
        
        let processor = RotateFixedProcessor::new().with_options(
            RoteteFixedProcessorOptions {
                strategy,
            },
        );
        
        let final_image = processor.process(
            image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}
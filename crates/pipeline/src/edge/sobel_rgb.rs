use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::edge::sobel_rgb::{SobelRgbProcessor, SobelRgbProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct SobelRgbPipelineOptions {
    use_fast_approximation: bool,
}

pub struct SobelRgbPipeline {
    options: SobelRgbPipelineOptions,
}

impl SobelRgbPipeline {
    pub fn new(options: SobelRgbPipelineOptions) -> Self {
        Self {
            options,
        }
    }
}

impl Pipeline for SobelRgbPipeline {
    fn run(&self, fast_image: FastImage, pipeline_progress: Arc<RwLock<PipelineProgress>>) -> FastImage {
        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.setup_combined(1);
        pipeline_progress_write.new_individual("SobelRgb".to_string());
        drop(pipeline_progress_write);
        
        let processor = SobelRgbProcessor::new().with_options(SobelRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
        });
        let final_image = processor.process(fast_image, pipeline_progress.read().unwrap().get_individual_progress("SobelRgb".to_string()));
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::edge::sobel::{SobelProcessor, SobelProcessorOptions};
use std::sync::{Arc, RwLock};

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
        fast_image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage {
        let pipeline_progress = pipeline_progress.unwrap_or_else(|| {
            Arc::new(RwLock::new(PipelineProgress::new()))
        });

        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.setup_combined(1);
        pipeline_progress_write.new_individual("Sobel".to_string());
        drop(pipeline_progress_write);

        let processor = SobelProcessor::new().with_options(SobelProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
        });
        let final_image = processor.process(
            fast_image,
            pipeline_progress
                .read()
                .unwrap()
                .get_individual_progress("Sobel".to_string()),
        );
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}

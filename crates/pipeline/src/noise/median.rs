use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::noise::median::{MedianProcessor, MedianProcessorOptions};
use std::sync::{Arc, RwLock};

pub struct MedianPipelineOptions {
    pub radius: usize,
}

pub struct MedianPipeline {
    options: MedianPipelineOptions,
}

impl MedianPipeline {
    pub fn new(options: MedianPipelineOptions) -> Self {
        Self { options }
    }
}

impl Pipeline for MedianPipeline {
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
        pipeline_progress_write.new_individual("Median".to_string());
        drop(pipeline_progress_write);

        let processor = MedianProcessor::new().with_options(MedianProcessorOptions {
            radius: self.options.radius,
        });
        let final_image = processor.process(
            fast_image,
            pipeline_progress
                .read()
                .unwrap()
                .get_current_individual_progress(),
        );
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}

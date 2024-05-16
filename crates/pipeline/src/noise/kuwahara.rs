use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::noise::kuwahara::{KuwaharaProcessor, KuwaharaProcessorOptions};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct KuwaharaPipelineOptions {
    pub radius: usize,
}

pub struct KuwaharaPipeline {
    options: KuwaharaPipelineOptions,
}

impl KuwaharaPipeline {
    pub fn new(options: KuwaharaPipelineOptions) -> Self {
        Self {
            options,
        }
    }
}

impl Pipeline for KuwaharaPipeline {
    fn run(&self, fast_image: FastImage, pipeline_progress: Arc<RwLock<PipelineProgress>>) -> FastImage {
        let mut pipeline_progress_write = pipeline_progress.write().unwrap();
        pipeline_progress_write.setup_combined(1);
        pipeline_progress_write.new_individual("Kuwahara".to_string());
        drop(pipeline_progress_write);

        let processor = KuwaharaProcessor::new().with_options(KuwaharaProcessorOptions {
            radius: self.options.radius,
        });
        let final_image = processor.process(fast_image, pipeline_progress.read().unwrap().get_individual_progress("Kuwahara".to_string()));
        pipeline_progress.write().unwrap().increment_combined();
        final_image
    }
}
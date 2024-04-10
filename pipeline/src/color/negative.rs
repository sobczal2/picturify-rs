use picturify_core::fast_image::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::negative::NegativeProcessor;
use crate::pipeline::Pipeline;

pub struct NegativePipelineOptions {}

pub struct NegativePipeline {
    processors: Vec<Box<dyn Processor>>,
    #[allow(dead_code)]
    options: NegativePipelineOptions,
}

impl NegativePipeline {
    pub fn new(_negative_pipeline_options: NegativePipelineOptions) -> Self {
        Self {
            processors: vec![Box::new(NegativeProcessor::new())],
            options: NegativePipelineOptions {},
        }
    }
}

impl Pipeline for NegativePipeline {
    fn run(&self, fast_image: FastImage) -> FastImage {
        let mut image = fast_image;
        for processor in &self.processors {
            image = processor.process(image);
        }
        image
    }
}

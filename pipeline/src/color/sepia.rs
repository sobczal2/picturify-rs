use picturify_core::fast_image::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;
use crate::pipeline::Pipeline;

pub struct SepiaPipelineOptions {}

pub struct SepiaPipeline {
    processors: Vec<Box<dyn Processor>>,
    #[allow(dead_code)]
    options: SepiaPipelineOptions,
}

impl SepiaPipeline {
    pub fn new(_sepia_pipeline_options: SepiaPipelineOptions) -> Self {
        Self {
            processors: vec![Box::new(SepiaProcessor::new())],
            options: SepiaPipelineOptions {},
        }
    }
}

impl Pipeline for SepiaPipeline {
    fn run(&self, fast_image: FastImage) -> FastImage {
        let mut image = fast_image;
        for processor in &self.processors {
            image = processor.process(image);
        }
        image
    }
}

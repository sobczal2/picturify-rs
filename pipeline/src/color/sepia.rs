use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;

pub struct SepiaPipelineOptions {}

pub struct SepiaPipeline {
    processors: Vec<Box<dyn Processor>>,
    #[allow(dead_code)]
    options: SepiaPipelineOptions,
}

impl Default for SepiaPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl SepiaPipeline {
    pub fn new() -> Self {
        Self {
            processors: vec![Box::new(SepiaProcessor::new())],
            options: SepiaPipelineOptions {},
        }
    }

    pub fn with_options(options: SepiaPipelineOptions) -> Self {
        Self {
            processors: vec![Box::new(SepiaProcessor::new())],
            options,
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

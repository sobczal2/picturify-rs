use picturify_core::image::fast_image::FastImage;
use picturify_macros::pipeline_run;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;

pub struct SepiaPipelineOptions {}

#[derive(pipeline_run)]
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

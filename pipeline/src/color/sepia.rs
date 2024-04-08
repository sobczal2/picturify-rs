use picturify_core::image::fast_image::FastImage;
use picturify_pipelines_macro::pipeline_run;
use picturify_processing::common::process::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;

pub struct SepiaOptions {}

#[derive(pipeline_run)]
pub struct SepiaPipeline {
    processors: Vec<Box<dyn Processor>>,
    #[allow(dead_code)]
    options: SepiaOptions,
}

impl SepiaPipeline {
    pub fn new(_sepia_options: SepiaOptions) -> Self {
        Self {
            processors: vec![Box::new(SepiaProcessor::new())],
            options: SepiaOptions {},
        }
    }
}

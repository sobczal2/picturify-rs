use picturify_core::image::fast_image::FastImage;
use picturify_macros::pipeline_run;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::negative::NegativeProcessor;

pub struct NegativePipelineOptions {}

#[derive(pipeline_run)]
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

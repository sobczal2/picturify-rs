use clap::ArgMatches;
use picturify_core::fast_image::FastImage;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use picturify_pipeline::pipeline::Pipeline;

pub struct NegativeCommandHandler;

impl NegativeCommandHandler {
    pub fn handle(fast_image: FastImage, _args: ArgMatches) -> FastImage {
        let negative_pipeline = NegativePipeline::new(NegativePipelineOptions {});
        
        negative_pipeline.run(fast_image)
    }
}

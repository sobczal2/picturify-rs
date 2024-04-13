use clap::ArgMatches;

use picturify_core::fast_image::FastImage;
use picturify_pipeline::color::sepia::SepiaPipeline;
use picturify_pipeline::pipeline::Pipeline;

pub struct SepiaCommandHandler;

impl SepiaCommandHandler {
    pub fn handle(fast_image: FastImage, _args: ArgMatches) -> FastImage {
        let sepia_pipeline = SepiaPipeline::new();
        
        sepia_pipeline.run(fast_image)
    }
}

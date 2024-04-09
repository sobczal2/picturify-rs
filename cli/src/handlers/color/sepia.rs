use clap::ArgMatches;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_pipeline::color::sepia::{SepiaOptions, SepiaPipeline};

pub struct SepiaCommandHandler;

impl SepiaCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let sepia_pipeline = SepiaPipeline::new(SepiaOptions {});
        let processed_image = sepia_pipeline.run(fast_image);
        processed_image
    }
}

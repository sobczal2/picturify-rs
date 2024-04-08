use clap::ArgMatches;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_pipeline::color::sepia::{SepiaOptions, SepiaPipeline};

pub struct SepiaCommandHandler;

impl SepiaCommandHandler {
    pub fn handle(args: ArgMatches) {
        let input = args.get_one::<String>("input").unwrap();
        let output = args.get_one::<String>("output").unwrap();

        let image = FastImage::read_from_file(input).unwrap();
        let sepia_pipeline = SepiaPipeline::new(SepiaOptions {});
        let processed_image = sepia_pipeline.run(*image);
        processed_image.write_to_file(output).unwrap();
    }
}

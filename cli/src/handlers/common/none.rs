use clap::ArgMatches;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};

pub struct NoneCommandHandler;

impl NoneCommandHandler {
    pub fn handle(args: ArgMatches) {
        let input = args.get_one::<String>("input").unwrap();
        let output = args.get_one::<String>("output").unwrap();

        let image = FastImage::read_from_file(input).unwrap();
        image.write_to_file(output).unwrap();
    }
}

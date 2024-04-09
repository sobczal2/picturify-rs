use crate::handlers::color::sepia::SepiaCommandHandler;
use crate::handlers::common::none::NoneCommandHandler;
use clap::ArgMatches;
use log::info;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};

pub struct ImageCommandHandler;

impl ImageCommandHandler {
    pub fn handle(arg_matches: ArgMatches) {
        let input = arg_matches.get_one::<String>("input").unwrap();
        let output = arg_matches.get_one::<String>("output").unwrap();

        let read_start = std::time::Instant::now();
        let image = FastImage::read_from_file(input).unwrap();
        let mut image = *image;
        let read_elapsed_ms = read_start.elapsed().as_millis();
        info!("Reading image took {}ms", read_elapsed_ms);
        
        match arg_matches.subcommand() {
            Some(("sepia", args)) => {
                let now = std::time::Instant::now();
                image = SepiaCommandHandler::handle(image, args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("Sepia took {}ms", elapsed_ms);
            }
            Some(("none", args)) => {
                let now = std::time::Instant::now();
                NoneCommandHandler::handle(args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("None took {}ms", elapsed_ms);
            }
            _ => {
                panic!("Unknown subcommand");
            }
        }
        
        let write_start = std::time::Instant::now();
        image.write_to_file(output).unwrap();
        let write_elapsed_ms = write_start.elapsed().as_millis();
        info!("Writing image took {}ms", write_elapsed_ms);
        
        let total_elapsed_ms = read_start.elapsed().as_millis();
        info!("Total time elapsed: {}ms", total_elapsed_ms);
    }
}

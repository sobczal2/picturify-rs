use std::time::Instant;
use crate::handlers::color::negative::NegativeCommandHandler;
use crate::handlers::color::sepia::SepiaCommandHandler;
use clap::ArgMatches;
use log::info;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use crate::handlers::noise::kuwahara::KuwaharaCommandHandler;
use crate::handlers::noise::median::MedianCommandHandler;

pub struct ImageCommandHandler;

impl ImageCommandHandler {
    pub fn handle(arg_matches: ArgMatches) {
        let input = arg_matches.get_one::<String>("input").unwrap();
        let output = arg_matches.get_one::<String>("output").unwrap();

        let read_start = Instant::now();
        let image = FastImage::read_from_file(input).unwrap();
        let mut image = *image;
        let read_elapsed_ms = read_start.elapsed().as_millis();
        info!("Reading fast_image took {}ms", read_elapsed_ms);

        match arg_matches.subcommand() {
            Some(("sepia", args)) => {
                let now = Instant::now();
                image = SepiaCommandHandler::handle(image, args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("Sepia took {}ms", elapsed_ms);
            }
            Some(("negative", args)) => {
                let now = Instant::now();
                image = NegativeCommandHandler::handle(image, args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("Negative took {}ms", elapsed_ms);
            }
            Some(("kuwahara", args)) => {
                let now = Instant::now();
                image = KuwaharaCommandHandler::handle(image, args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("Kuwahara took {}ms", elapsed_ms);
            }
            Some(("median", args)) => {
                let now = Instant::now();
                image = MedianCommandHandler::handle(image, args.clone());
                let elapsed_ms = now.elapsed().as_millis();
                info!("Median took {}ms", elapsed_ms);
            }
            Some(("none", _args)) => {}
            _ => {
                panic!("Unknown subcommand");
            }
        }

        let write_start = Instant::now();
        image.write_to_file(output).unwrap();
        let write_elapsed_ms = write_start.elapsed().as_millis();
        info!("Writing fast_image took {}ms", write_elapsed_ms);

        let total_elapsed_ms = read_start.elapsed().as_millis();
        info!("Total time elapsed: {}ms", total_elapsed_ms);
    }
}

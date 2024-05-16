use clap::ArgMatches;
use log::{error, info};
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_core::fast_image::FastImage;
use std::time::Instant;
use crate::handlers::image::color::negative::NegativeCommandHandler;
use crate::handlers::image::color::sepia::SepiaCommandHandler;
use crate::handlers::image::edge::sobel::SobelCommandHandler;
use crate::handlers::image::edge::sobel_rgb::SobelRgbCommandHandler;
use crate::handlers::image::noise::kuwahara::KuwaharaCommandHandler;
use crate::handlers::image::noise::median::MedianCommandHandler;

pub struct ImageCommandHandler;

impl ImageCommandHandler {
    pub fn handle(arg_matches: ArgMatches) {
        let input = arg_matches.get_one::<String>("input").unwrap();
        let output = arg_matches.get_one::<String>("output").unwrap();

        let read_start = Instant::now();
        let image = FastImage::read_from_file(input);
        let image = match image {
            Ok(image) => image,
            Err(e) => {
                error!("Failed to read file: {}", e);
                return;
            }
        };
        let mut image = *image;
        let read_elapsed_ms = read_start.elapsed().as_millis();
        info!("Reading fast_image took {}ms", read_elapsed_ms);

        match arg_matches.subcommand() {
            Some(("none", _args)) => {}

            // color
            Some(("sepia", args)) => {
                image = Self::log_time("Sepia", || {
                    SepiaCommandHandler::handle(image, args.clone())
                });
            }
            Some(("negative", args)) => {
                image = Self::log_time("Negative", || {
                    NegativeCommandHandler::handle(image, args.clone())
                });
            }

            // noise
            Some(("kuwahara", args)) => {
                image = Self::log_time("Kuwahara", || {
                    KuwaharaCommandHandler::handle(image, args.clone())
                });
            }
            Some(("median", args)) => {
                image = Self::log_time("Median", || {
                    MedianCommandHandler::handle(image, args.clone())
                });
            }

            // edge
            Some(("sobel", args)) => {
                image = Self::log_time("Sobel", || {
                    SobelCommandHandler::handle(image, args.clone())
                });
            }
            Some(("sobel_rgb", args)) => {
                image = Self::log_time("Sobel RGB", || {
                    SobelRgbCommandHandler::handle(image, args.clone())
                });
            }
            _ => {
                error!("Command not found");
                return;
            }
        }

        let write_start = Instant::now();
        let result = image.write_to_file(output);
        if let Err(e) = result {
            error!("Failed to write file: {}", e);
            return;
        }
        let write_elapsed_ms = write_start.elapsed().as_millis();
        info!("Writing fast_image took {}ms", write_elapsed_ms);

        let total_elapsed_ms = read_start.elapsed().as_millis();
        info!("Total time elapsed: {}ms", total_elapsed_ms);
    }

    fn log_time<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
        let now = Instant::now();
        let result = f();
        let elapsed_ms = now.elapsed().as_millis();
        info!("{} took {}ms", name, elapsed_ms);
        result
    }
}

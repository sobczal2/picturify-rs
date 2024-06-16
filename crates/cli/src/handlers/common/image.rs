use std::collections::HashMap;

use clap::ArgMatches;

use crate::commands::common::command::{Command, CommandForImage};
use crate::commands::common::image::ImageCommand;
use crate::commands::image::blob::laplacian_of_gaussian::LaplacianOfGaussianCommand;
use crate::commands::image::color::brightness::BrightnessCommand;
use crate::commands::image::color::grayscale::GrayscaleCommand;
use crate::commands::image::color::negative::NegativeCommand;
use crate::commands::image::color::quantization::QuantizationCommand;
use crate::commands::image::color::sepia::SepiaCommand;
use crate::commands::image::common::passthrough::PassthroughCommand;
use crate::commands::image::edge::prewitt::PrewittCommand;
use crate::commands::image::edge::sobel::SobelCommand;
use crate::commands::image::geometry::crop::CropCommand;
use crate::commands::image::geometry::rotate::RotateCommand;
use crate::commands::image::geometry::scale::ScaleCommand;
use crate::commands::image::noise::bilateral_blur::BilateralBlurCommand;
use crate::commands::image::noise::gaussian_blur::GaussianBlurCommand;
use crate::commands::image::noise::kuwahara::KuwaharaCommand;
use crate::commands::image::noise::mean_blur::MeanBlurCommand;
use crate::commands::image::noise::median_blur::MedianBlurCommand;
use crate::commands::image::noise::sharpen::SharpenCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::image::blob::laplacian_of_gaussian::LaplacianOfGaussianCommandHandler;
use crate::handlers::image::color::brightness::BrightnessCommandHandler;
use crate::handlers::image::color::grayscale::GrayscaleCommandHandler;
use crate::handlers::image::color::negative::NegativeCommandHandler;
use crate::handlers::image::color::quantization::QuantizationCommandHandler;
use crate::handlers::image::color::sepia::SepiaCommandHandler;
use crate::handlers::image::common::passthrough::PassthroughCommandHandler;
use crate::handlers::image::edge::prewitt::PrewittCommandHandler;
use crate::handlers::image::edge::sobel::SobelCommandHandler;
use crate::handlers::image::geometry::crop::CropCommandHandler;
use crate::handlers::image::geometry::rotate::RotateCommandHandler;
use crate::handlers::image::geometry::scale::ScaleCommandHandler;
use crate::handlers::image::noise::bilateral_blur::BilateralBlurCommandHandler;
use crate::handlers::image::noise::gaussian_blur::GaussianBlurCommandHandler;
use crate::handlers::image::noise::kuwahara::KuwaharaCommandHandler;
use crate::handlers::image::noise::mean_blur::MeanBlurCommandHandler;
use crate::handlers::image::noise::median_blur::MedianBlurCommandHandler;
use crate::handlers::image::noise::sharpen::SharpenCommandHandler;

pub struct ImageCommandHandler;

impl CommandHandler for ImageCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Box<dyn CommandHandler>> = HashMap::new();

                // blob
                handlers.insert(LaplacianOfGaussianCommand::name(), Box::new(LaplacianOfGaussianCommandHandler));

                // common
                handlers.insert(
                    PassthroughCommand::name(),
                    Box::new(PassthroughCommandHandler),
                );

                // color
                handlers.insert(SepiaCommand::name(), Box::new(SepiaCommandHandler));
                handlers.insert(NegativeCommand::name(), Box::new(NegativeCommandHandler));
                handlers.insert(GrayscaleCommand::name(), Box::new(GrayscaleCommandHandler));
                handlers.insert(
                    BrightnessCommand::name(),
                    Box::new(BrightnessCommandHandler),
                );
                handlers.insert(
                    QuantizationCommand::name(),
                    Box::new(QuantizationCommandHandler),
                );

                // noise
                handlers.insert(KuwaharaCommand::name(), Box::new(KuwaharaCommandHandler));
                handlers.insert(
                    MedianBlurCommand::name(),
                    Box::new(MedianBlurCommandHandler),
                );
                handlers.insert(MeanBlurCommand::name(), Box::new(MeanBlurCommandHandler));
                handlers.insert(SharpenCommand::name(), Box::new(SharpenCommandHandler));
                handlers.insert(
                    GaussianBlurCommand::name(),
                    Box::new(GaussianBlurCommandHandler),
                );
                handlers.insert(
                    BilateralBlurCommand::name(),
                    Box::new(BilateralBlurCommandHandler),
                );

                // edge
                handlers.insert(SobelCommand::name(), Box::new(SobelCommandHandler));
                handlers.insert(PrewittCommand::name(), Box::new(PrewittCommandHandler));

                // geometry
                handlers.insert(RotateCommand::name(), Box::new(RotateCommandHandler));
                handlers.insert(CropCommand::name(), Box::new(CropCommandHandler));
                handlers.insert(ScaleCommand::name(), Box::new(ScaleCommandHandler));

                if let Some(handler) = handlers.get(name) {
                    handler.handle(args.clone())
                } else {
                    Err(CliPicturifyError::InvalidCommand(name.to_string()))
                }
            }
            None => {
                ImageCommand::get().print_help().unwrap();
                Err(CliPicturifyError::MissingSubcommand)
            }
        }
    }
}

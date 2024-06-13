use crate::commands::common::command::Command;
use crate::commands::common::image::ImageCommand;
use crate::commands::image::color::brightness::BrightnessCommand;
use crate::commands::image::color::grayscale::GrayscaleCommand;
use crate::commands::image::color::negative::NegativeCommand;
use crate::commands::image::color::sepia::SepiaCommand;
use crate::commands::image::common::none::NoneCommand;
use crate::commands::image::edge::sobel::SobelCommand;
use crate::commands::image::geometry::crop::CropCommand;
use crate::commands::image::geometry::rotate::RotateCommand;
use crate::commands::image::noise::bilateral_blur::BilateralBlurCommand;
use crate::commands::image::noise::gaussian_blur::GaussianBlurCommand;
use crate::commands::image::noise::kuwahara::KuwaharaCommand;
use crate::commands::image::noise::mean_blur::MeanBlurCommand;
use crate::commands::image::noise::median_blur::MedianBlurCommand;
use crate::commands::image::noise::sharpen::SharpenCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::image::color::brightness::BrightnessCommandHandler;
use crate::handlers::image::color::grayscale::GrayscaleCommandHandler;
use crate::handlers::image::color::negative::NegativeCommandHandler;
use crate::handlers::image::color::sepia::SepiaCommandHandler;
use crate::handlers::image::common::none::NoneCommandHandler;
use crate::handlers::image::edge::sobel::SobelCommandHandler;
use crate::handlers::image::geometry::crop::CropCommandHandler;
use crate::handlers::image::geometry::rotate::RotateCommandHandler;
use crate::handlers::image::noise::bilateral_blur::BilateralBlurCommandHandler;
use crate::handlers::image::noise::gaussian_blur::GaussianBlurCommandHandler;
use crate::handlers::image::noise::kuwahara::KuwaharaCommandHandler;
use crate::handlers::image::noise::mean_blur::MeanBlurCommandHandler;
use crate::handlers::image::noise::median_blur::MedianBlurCommandHandler;
use crate::handlers::image::noise::sharpen::SharpenCommandHandler;
use clap::ArgMatches;
use std::collections::HashMap;

pub struct ImageCommandHandler;

impl CommandHandler for ImageCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Box<dyn CommandHandler>> = HashMap::new();

                // common
                handlers.insert(NoneCommand::name(), Box::new(NoneCommandHandler));

                // color
                handlers.insert(SepiaCommand::name(), Box::new(SepiaCommandHandler));
                handlers.insert(NegativeCommand::name(), Box::new(NegativeCommandHandler));
                handlers.insert(GrayscaleCommand::name(), Box::new(GrayscaleCommandHandler));
                handlers.insert(
                    BrightnessCommand::name(),
                    Box::new(BrightnessCommandHandler),
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

                // geometry
                handlers.insert(RotateCommand::name(), Box::new(RotateCommandHandler));
                handlers.insert(CropCommand::name(), Box::new(CropCommandHandler));

                if let Some(handler) = handlers.get(name) {
                    handler.handle(args.clone())
                } else {
                    Err(CliPicturifyError::InvalidCommand(name.to_string()))
                }
            }
            None => {
                ImageCommand::get().print_help().unwrap();
                Err(CliPicturifyError::MissingCommand)
            }
        }
    }
}

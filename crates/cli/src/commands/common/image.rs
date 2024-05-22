use crate::commands::common::command::Command;
use crate::commands::image::color::brightness::BrightnessCommand;
use crate::commands::image::color::grayscale::GrayscaleCommand;
use crate::commands::image::color::negative::NegativeCommand;
use crate::commands::image::color::sepia::SepiaCommand;
use crate::commands::image::common::none::NoneCommand;
use crate::commands::image::edge::sobel::SobelCommand;
use crate::commands::image::edge::sobel_rgb::SobelRgbCommand;
use crate::commands::image::geometry::crop::CropCommand;
use crate::commands::image::geometry::rotate::RotateCommand;
use crate::commands::image::noise::bilateral::BilateralCommand;
use crate::commands::image::noise::gaussian_blur::GaussianBlurCommand;
use crate::commands::image::noise::kuwahara::KuwaharaCommand;
use crate::commands::image::noise::mean::MeanCommand;
use crate::commands::image::noise::median::MedianCommand;
use crate::commands::image::noise::sharpen::SharpenCommand;

pub struct ImageCommand;

impl Command for ImageCommand {
    fn get() -> clap::Command {
        clap::Command::new(Self::name())
            .about("Run image processing-bench pipeline on an image")
            .subcommands(&[
                // common
                NoneCommand::get(),
                // color
                SepiaCommand::get(),
                NegativeCommand::get(),
                GrayscaleCommand::get(),
                BrightnessCommand::get(),
                // noise
                KuwaharaCommand::get(),
                MedianCommand::get(),
                MeanCommand::get(),
                SharpenCommand::get(),
                GaussianBlurCommand::get(),
                BilateralCommand::get(),
                // edge
                SobelCommand::get(),
                SobelRgbCommand::get(),
                // geometry
                RotateCommand::get(),
                CropCommand::get(),
            ])
    }

    fn name() -> &'static str {
        "image"
    }
}

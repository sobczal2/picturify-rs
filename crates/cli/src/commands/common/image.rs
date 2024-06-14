use crate::commands::common::command::{Command, CommandForImage};
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

pub struct ImageCommand;

impl Command for ImageCommand {
    fn get() -> clap::Command {
        clap::Command::new("image")
            .about("Run processing pipeline on the image")
            .subcommands(&[
                // common
                PassthroughCommand::get(),
                // color
                SepiaCommand::get(),
                NegativeCommand::get(),
                GrayscaleCommand::get(),
                BrightnessCommand::get(),
                QuantizationCommand::get(),
                // noise
                KuwaharaCommand::get(),
                MedianBlurCommand::get(),
                MeanBlurCommand::get(),
                SharpenCommand::get(),
                GaussianBlurCommand::get(),
                BilateralBlurCommand::get(),
                // edge
                SobelCommand::get(),
                PrewittCommand::get(),
                // geometry
                RotateCommand::get(),
                CropCommand::get(),
                ScaleCommand::get(),
            ])
    }
}

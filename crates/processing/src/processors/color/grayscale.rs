use crate::common::execution::Processor;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::coord::Coord;
use picturify_core::image::Rgba;
use picturify_core::palette::LinSrgba;
use picturify_core::threading::progress::Progress;

#[derive(Clone, Copy)]
pub enum GrayscaleStrategy {
    Average,
    Lightness,
    Luminosity,
}

impl ValueEnum for GrayscaleStrategy {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GrayscaleStrategy::Average,
            GrayscaleStrategy::Lightness,
            GrayscaleStrategy::Luminosity,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            GrayscaleStrategy::Average => Some(PossibleValue::new("average")),
            GrayscaleStrategy::Lightness => Some(PossibleValue::new("lightness")),
            GrayscaleStrategy::Luminosity => Some(PossibleValue::new("luminosity")),
        }
    }
}

pub struct GrayscaleProcessorOptions {
    pub strategy: GrayscaleStrategy,
    pub use_fast_approximation: bool,
}

pub struct GrayscaleProcessor {
    options: GrayscaleProcessorOptions,
}

impl GrayscaleProcessor {
    pub fn new(options: GrayscaleProcessorOptions) -> Self {
        GrayscaleProcessor { options }
    }
}

impl Processor for GrayscaleProcessor {
    fn process(&self, mut image: FastImage, progress: Progress) -> FastImage {
        if self.options.use_fast_approximation {
            let function = match self.options.strategy {
                GrayscaleStrategy::Average => {
                    GrayscaleProcessor::average_processing_function_fast()
                }
                GrayscaleStrategy::Lightness => {
                    GrayscaleProcessor::lightness_processing_function_fast()
                }
                GrayscaleStrategy::Luminosity => {
                    GrayscaleProcessor::luminosity_processing_function_fast()
                }
            };
            image.par_apply_fn_to_image_pixel(function, Some(progress));
        } else {
            let function = match self.options.strategy {
                GrayscaleStrategy::Average => GrayscaleProcessor::average_processing_function(),
                GrayscaleStrategy::Lightness => GrayscaleProcessor::lightness_processing_function(),
                GrayscaleStrategy::Luminosity => {
                    GrayscaleProcessor::luminosity_processing_function()
                }
            };
            image.par_apply_fn_to_lin_srgba(function, Some(progress));
        }
        image
    }
}

impl GrayscaleProcessor {
    fn average_processing_function_fast() -> Box<dyn Fn(&mut Rgba<u8>, Coord) + Send + Sync> {
        Box::new(|pixel, _coord| {
            let avg = (pixel.0[0] as f32 + pixel.0[1] as f32 + pixel.0[2] as f32) / 3.0;
            for i in 0..3 {
                pixel.0[i] = avg as u8;
            }
        })
    }

    fn lightness_processing_function_fast() -> Box<dyn Fn(&mut Rgba<u8>, Coord) + Send + Sync> {
        Box::new(|pixel, _coord| {
            let max = pixel.0.iter().max().unwrap();
            let min = pixel.0.iter().min().unwrap();
            let avg = (*max as f32 + *min as f32) / 2.0;
            for i in 0..3 {
                pixel.0[i] = avg as u8;
            }
        })
    }

    fn luminosity_processing_function_fast() -> Box<dyn Fn(&mut Rgba<u8>, Coord) + Send + Sync> {
        Box::new(|pixel, _coord| {
            let avg =
                0.21 * pixel.0[0] as f32 + 0.72 * pixel.0[1] as f32 + 0.07 * pixel.0[2] as f32;
            for i in 0..3 {
                pixel.0[i] = avg as u8;
            }
        })
    }

    fn average_processing_function() -> Box<dyn Fn(LinSrgba, Coord) -> LinSrgba + Send + Sync> {
        Box::new(|mut pixel, _coord| {
            let avg = (pixel.red + pixel.green + pixel.blue) / 3.0;
            pixel.red = avg;
            pixel.green = avg;
            pixel.blue = avg;

            pixel
        })
    }

    fn lightness_processing_function() -> Box<dyn Fn(LinSrgba, Coord) -> LinSrgba + Send + Sync> {
        Box::new(|mut pixel, _coord| {
            let max = pixel.red.max(pixel.green).max(pixel.blue);
            let min = pixel.red.min(pixel.green).min(pixel.blue);
            let avg = (max + min) / 2.0;
            pixel.red = avg;
            pixel.green = avg;
            pixel.blue = avg;

            pixel
        })
    }

    fn luminosity_processing_function() -> Box<dyn Fn(LinSrgba, Coord) -> LinSrgba + Send + Sync> {
        Box::new(|mut pixel, _coord| {
            let avg = 0.21 * pixel.red + 0.72 * pixel.green + 0.07 * pixel.blue;
            pixel.red = avg;
            pixel.green = avg;
            pixel.blue = avg;

            pixel
        })
    }
}

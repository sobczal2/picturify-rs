use picturify_core::core::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::palette::LinSrgba;
use picturify_core::threading::progress::Progress;

use crate::common::execution::Processor;

pub enum RemappingFunction {
    Linear {
        min: f32,
        max: f32,
    },
    Exponential {
        factor: f32,
    },
    Logarithmic {
        factor: f32,
    },
    Custom {
        map: fn(lin_srgb: LinSrgba) -> LinSrgba,
    },
}

impl RemappingFunction {
    fn apply_to_pixel(&self, pixel: LinSrgba) -> LinSrgba {
        match self {
            RemappingFunction::Linear { min, max } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = (r - min) / (max - min);
                let new_g = (g - min) / (max - min);
                let new_b = (b - min) / (max - min);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Exponential { factor } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = r.powf(*factor);
                let new_g = g.powf(*factor);
                let new_b = b.powf(*factor);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Logarithmic { factor } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = (r + 1.).log(*factor);
                let new_g = (g + 1.).log(*factor);
                let new_b = (b + 1.).log(*factor);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Custom { map } => map(pixel),
        }
    }
}

pub struct RemappingProcessorOptions {
    pub function: RemappingFunction,
}

pub struct RemappingProcessor {
    options: RemappingProcessorOptions,
}

impl RemappingProcessor {
    pub fn new(options: RemappingProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for RemappingProcessor {
    fn process(&self, mut image: FastImage, progress: Progress) -> ProcessingPicturifyResult<FastImage> {
        image.par_apply_fn_to_lin_srgba(
            |pixel, _coord| self.options.function.apply_to_pixel(pixel),
            Some(progress),
        );

        Ok(image)
    }
}

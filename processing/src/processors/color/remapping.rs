use std::sync::{Arc, RwLock};
use crate::common::execution::{Processor};
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;

use picturify_core::palette::{LinSrgba};
use picturify_core::threading::progress::Progress;

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

impl Default for RemappingProcessorOptions {
    fn default() -> RemappingProcessorOptions {
        RemappingProcessorOptions {
            function: RemappingFunction::Linear { min: 0.0, max: 1.0 },
        }
    }
}

pub struct RemappingProcessor {
    options: RemappingProcessorOptions,
}

impl Processor<RemappingProcessorOptions> for RemappingProcessor {
    fn new() -> Self {
        RemappingProcessor {
            options: Default::default(),
        }
    }

    fn with_options(self, options: RemappingProcessorOptions) -> Self {
        RemappingProcessor {
            options,
        }
    }

    fn process(&self, mut fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        fast_image.par_apply_fn_to_lin_srgba(
            |pixel, _x, _y| {
                self.options.function.apply_to_pixel(pixel)
            },
            Some(progress),
        );

        fast_image
    }
}

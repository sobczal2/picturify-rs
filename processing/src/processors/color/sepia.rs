use std::sync::{Arc, RwLock};
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor};

pub struct SepiaProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for SepiaProcessorOptions {
    fn default() -> Self {
        SepiaProcessorOptions {
            use_fast_approximation: true,
        }
    }
}

pub struct SepiaProcessor {
    options: SepiaProcessorOptions,
}

impl Processor<SepiaProcessorOptions> for SepiaProcessor {
    fn new() -> Self {
        SepiaProcessor {
            options: Default::default(),
        }
    }

    fn with_options(self, options: SepiaProcessorOptions) -> Self {
        SepiaProcessor { options }
    }

    fn process(&self, mut fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        return if self.options.use_fast_approximation {
            fast_image.par_apply_fn_to_image_pixel(
                |pixel, _x, _y| {
                    let r = pixel.0[0] as f32;
                    let g = pixel.0[1] as f32;
                    let b = pixel.0[2] as f32;

                    let new_r = r * 0.393 + g * 0.769 + b * 0.189;
                    let new_g = r * 0.349 + g * 0.686 + b * 0.168;
                    let new_b = r * 0.272 + g * 0.534 + b * 0.131;

                    pixel.0[0] = new_r.round() as u8;
                    pixel.0[1] = new_g.round() as u8;
                    pixel.0[2] = new_b.round() as u8;
                },
                Some(progress),
            );
            fast_image
        } else {
            fast_image.par_apply_fn_to_lin_srgba(
                |mut pixel, _x, _y| {
                    let r = pixel.red;
                    let g = pixel.green;
                    let b = pixel.blue;

                    let new_r = r * 0.393 + g * 0.769 + b * 0.189;
                    let new_g = r * 0.349 + g * 0.686 + b * 0.168;
                    let new_b = r * 0.272 + g * 0.534 + b * 0.131;

                    pixel.red = new_r;
                    pixel.green = new_g;
                    pixel.blue = new_b;

                    pixel
                },
                Some(progress),
            );
            fast_image
        };
    }
}

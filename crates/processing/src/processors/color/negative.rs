use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct NegativeProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for NegativeProcessorOptions {
    fn default() -> Self {
        NegativeProcessorOptions {
            use_fast_approximation: true,
        }
    }
}

pub struct NegativeProcessor {
    options: NegativeProcessorOptions,
}

impl NegativeProcessor {
    pub fn new() -> Self {
        NegativeProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<NegativeProcessorOptions> for NegativeProcessor {
    fn with_options(self, options: NegativeProcessorOptions) -> Self {
        NegativeProcessor { options }
    }
}

impl Processor for NegativeProcessor {
    fn process(&self, mut image: FastImage, progress: Progress) -> FastImage {
        if self.options.use_fast_approximation {
            image.par_apply_fn_to_image_pixel(
                |pixel, _coord| {
                    pixel.0[0] = 255 - pixel.0[0];
                    pixel.0[1] = 255 - pixel.0[1];
                    pixel.0[2] = 255 - pixel.0[2];
                },
                Some(progress),
            );
        } else {
            image.par_apply_fn_to_lin_srgba(
                |mut pixel, _coord| {
                    pixel.red = 1.0 - pixel.red;
                    pixel.green = 1.0 - pixel.green;
                    pixel.blue = 1.0 - pixel.blue;
                    pixel
                },
                Some(progress),
            );
        }

        image
    }
}

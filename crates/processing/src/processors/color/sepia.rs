use picturify_core::core::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

use crate::common::execution::Processor;

pub struct SepiaProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct SepiaProcessor {
    options: SepiaProcessorOptions,
}

impl SepiaProcessor {
    pub fn new(options: SepiaProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for SepiaProcessor {
    fn process(&self, mut image: FastImage, progress: Progress) -> ProcessingPicturifyResult<FastImage> {
        if self.options.use_fast_approximation {
            image.par_apply_fn_to_image_pixel(
                |pixel, _coord| {
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
            Ok(image)
        } else {
            image.par_apply_fn_to_lin_srgba(
                |mut pixel, _coord| {
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
            Ok(image)
        }
    }
}

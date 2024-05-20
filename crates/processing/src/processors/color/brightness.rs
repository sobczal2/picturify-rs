use std::sync::{Arc, RwLock};

use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Hsla;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor, WithOptions};

pub struct BrightnessProcessorOptions {
    pub factor: f32,
}

impl Default for BrightnessProcessorOptions {
    fn default() -> Self {
        BrightnessProcessorOptions { factor: 1.0 }
    }
}

pub struct BrightnessProcessor {
    options: BrightnessProcessorOptions,
}

impl BrightnessProcessor {
    pub fn new() -> Self {
        BrightnessProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<BrightnessProcessorOptions> for BrightnessProcessor {
    fn with_options(self, options: BrightnessProcessorOptions) -> Self {
        BrightnessProcessor { options }
    }
}

impl Processor for BrightnessProcessor {
    fn process(&self, mut image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        image.par_apply_fn_to_pixel(
            |mut pixel: Hsla, _x, _y| {
                pixel.lightness = (pixel.lightness * self.options.factor).max(0.0).min(1.0);
                pixel
            },
            Some(progress),
        );
        
        image
    }
}
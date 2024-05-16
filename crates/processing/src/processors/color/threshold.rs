use std::sync::{Arc, RwLock};
use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct ThresholdProcessorOptions {
    pub red_threshold: u8,
    pub green_threshold: u8,
    pub blue_threshold: u8,
}

impl Default for ThresholdProcessorOptions {
    fn default() -> ThresholdProcessorOptions {
        ThresholdProcessorOptions {
            red_threshold: 128,
            green_threshold: 128,
            blue_threshold: 128,
        }
    }
}

pub struct ThresholdProcessor {
    options: ThresholdProcessorOptions,
}

impl ThresholdProcessor {
    pub fn new() -> Self {
        ThresholdProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<ThresholdProcessorOptions> for ThresholdProcessor {
    fn with_options(self, options: ThresholdProcessorOptions) -> Self {
        ThresholdProcessor { options }
    }
}

impl Processor for ThresholdProcessor {
    fn process(&self, mut fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        fast_image.par_apply_fn_to_image_pixel(
            |pixel, _x, _y| {
                pixel.0[0] = if pixel.0[0] > self.options.red_threshold {
                    pixel.0[0]
                } else {
                    0
                };
                pixel.0[1] = if pixel.0[1] > self.options.green_threshold {
                    pixel.0[1]
                } else {
                    0
                };
                pixel.0[2] = if pixel.0[2] > self.options.blue_threshold {
                    pixel.0[2]
                } else {
                    0
                };
            },
            Some(progress),
        );

        fast_image
    }
}
use crate::common::execution::Processor;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct ThresholdProcessorOptions {
    pub red_threshold: u8,
    pub green_threshold: u8,
    pub blue_threshold: u8,
}

pub struct ThresholdProcessor {
    options: ThresholdProcessorOptions,
}

impl ThresholdProcessor {
    pub fn new(options: ThresholdProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for ThresholdProcessor {
    fn process(&self, mut image: FastImage, progress: Progress) -> FastImage {
        image.par_apply_fn_to_image_pixel(
            |pixel, _coord| {
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

        image
    }
}

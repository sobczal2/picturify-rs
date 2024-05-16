use std::sync::{Arc, RwLock};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor, WithOptions};

pub struct CropProcessorOptions {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for CropProcessorOptions {
    fn default() -> Self {
        CropProcessorOptions {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

pub struct CropProcessor {
    options: CropProcessorOptions,
}

impl CropProcessor {
    pub fn new() -> Self {
        CropProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<CropProcessorOptions> for CropProcessor {
    fn with_options(self, options: CropProcessorOptions) -> Self {
        CropProcessor { options }
    }
}

impl Processor for CropProcessor {
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let mut new_image = FastImage::empty(self.options.width, self.options.height);

        new_image.par_apply_fn_to_image_pixel(
            |pixel, x, y| {
                let new_pixel = fast_image.get_image_pixel(x + self.options.x, y + self.options.y);
                *pixel = new_pixel;
            },
            Some(progress),
        );

        new_image
    }
}

use std::sync::{Arc, RwLock};
use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_core::threading::progress::Progress;

pub enum EnlargementStrategy {
    Constant(Srgba),
}

pub struct EnlargementProcessor {
    options: EnlargementProcessorOptions,
}

impl Default for EnlargementProcessorOptions {
    fn default() -> Self {
        EnlargementProcessorOptions {
            border: 0,
            strategy: EnlargementStrategy::Constant(Srgba::new(0f32, 0f32, 0f32, 0f32)),
        }
    }
}

pub struct EnlargementProcessorOptions {
    pub border: usize,
    pub strategy: EnlargementStrategy,
}

impl EnlargementProcessor {
    pub fn new() -> Self {
        EnlargementProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<EnlargementProcessorOptions> for EnlargementProcessor {
    fn with_options(self, options: EnlargementProcessorOptions) -> Self {
        EnlargementProcessor { options }
    }
}

impl Processor for EnlargementProcessor {
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let new_width = fast_image.get_width() + self.options.border * 2;
        let new_height = fast_image.get_height() + self.options.border * 2;

        let mut new_image = FastImage::empty(new_width, new_height);

        match self.options.strategy {
            EnlargementStrategy::Constant(pixel) => {
                new_image.par_apply_fn_to_pixel(
                    |_, x, y| {
                        if x < self.options.border
                            || x >= new_width - self.options.border
                            || y < self.options.border
                            || y >= new_height - self.options.border
                        {
                            pixel
                        } else {
                            fast_image
                                .get_srgba_pixel(x - self.options.border, y - self.options.border)
                        }
                    },
                    Some(progress),
                );
            }
        }

        new_image
    }
}

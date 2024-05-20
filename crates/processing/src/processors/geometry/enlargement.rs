use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_core::threading::progress::Progress;
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone)]
pub enum EnlargementStrategy {
    Constant(Srgba),
}

#[derive(Copy, Clone)]
pub struct EnlargementBorder {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Default for EnlargementBorder {
    fn default() -> Self {
        EnlargementBorder {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl EnlargementBorder {
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        EnlargementBorder {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_all(all: usize) -> Self {
        EnlargementBorder {
            top: all,
            right: all,
            bottom: all,
            left: all,
        }
    }

    pub fn from_x_y(x: usize, y: usize) -> Self {
        EnlargementBorder {
            top: y,
            right: x,
            bottom: y,
            left: x,
        }
    }
}

pub struct EnlargementProcessor {
    options: EnlargementProcessorOptions,
}

impl Default for EnlargementProcessorOptions {
    fn default() -> Self {
        EnlargementProcessorOptions {
            border: Default::default(),
            strategy: EnlargementStrategy::Constant(Srgba::new(0f32, 0f32, 0f32, 0f32)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct EnlargementProcessorOptions {
    pub border: EnlargementBorder,
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
    fn process(&self, image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let new_width = image.get_width() + self.options.border.left + self.options.border.right;
        let new_height = image.get_height() + self.options.border.top + self.options.border.bottom;

        let mut new_image = FastImage::empty(new_width, new_height);

        match self.options.strategy {
            EnlargementStrategy::Constant(pixel) => {
                new_image.par_apply_fn_to_pixel(
                    |_, x, y| {
                        if x < self.options.border.left
                            || x >= new_width - self.options.border.right
                            || y < self.options.border.top
                            || y >= new_height - self.options.border.bottom
                        {
                            pixel
                        } else {
                            image.get_srgba_pixel(
                                x - self.options.border.left,
                                y - self.options.border.top,
                            )
                        }
                    },
                    Some(progress),
                );
            }
        }

        new_image
    }
}

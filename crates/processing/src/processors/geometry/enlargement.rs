use crate::common::execution::Processor;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::palette::Srgba;
use picturify_core::threading::progress::Progress;

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

    pub fn is_inside(&self, coord: Coord, size: Size) -> bool {
        let (x, y): (usize, usize) = coord.into();
        let (width, height): (usize, usize) = size.into();
        x >= self.left && x < width - self.right && y >= self.top && y < height - self.bottom
    }
}

#[derive(Copy, Clone)]
pub struct EnlargementProcessorOptions {
    pub border: EnlargementBorder,
    pub strategy: EnlargementStrategy,
}

pub struct EnlargementProcessor {
    options: EnlargementProcessorOptions,
}

impl EnlargementProcessor {
    pub fn new(options: EnlargementProcessorOptions) -> Self {
        EnlargementProcessor { options }
    }
}

impl Processor for EnlargementProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let new_size = image.size().increase_by(
            self.options.border.left + self.options.border.right,
            self.options.border.top + self.options.border.bottom,
        );

        let mut new_image = FastImage::empty(new_size);
        let shift = (self.options.border.left, self.options.border.top).into();

        match self.options.strategy {
            EnlargementStrategy::Constant(pixel) => {
                new_image.par_apply_fn_to_pixel(
                    |_, coord| {
                        if self.options.border.is_inside(coord, new_size) {
                            image.get_srgba_pixel(coord - shift)
                        } else {
                            pixel
                        }
                    },
                    Some(progress),
                );
            }
        }

        new_image
    }
}

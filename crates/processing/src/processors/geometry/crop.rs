use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor, WithOptions};

#[derive(Copy, Clone)]
pub struct CropBorder {
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub y_offset: usize,
}

impl Default for CropBorder {
    fn default() -> Self {
        CropBorder {
            width: 0,
            height: 0,
            x_offset: 0,
            y_offset: 0,
        }
    }
}

impl CropBorder {
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
        CropBorder {
            width,
            height,
            x_offset,
            y_offset,
        }
    }
    
    pub fn offset(&self) -> Coord {
        (self.x_offset, self.y_offset).into()
    }
}

impl From<CropBorder> for Size {
    fn from(crop_border: CropBorder) -> Self {
        Size::new(crop_border.width, crop_border.height)
    }
}

#[derive(Copy, Clone)]
pub struct CropProcessorOptions {
    pub crop_border: CropBorder,
}

impl Default for CropProcessorOptions {
    fn default() -> Self {
        CropProcessorOptions {
            crop_border: Default::default(),
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
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let mut new_image = FastImage::empty(self.options.crop_border.into());

        new_image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                let new_pixel = image.get_image_pixel(coord + self.options.crop_border.offset());
                *pixel = new_pixel;
            },
            Some(progress),
        );

        new_image
    }
}

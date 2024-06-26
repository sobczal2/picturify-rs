use serde::{Deserialize, Serialize};
use picturify_core::core::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

use crate::common::processors::CpuProcessor;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct CropBorder {
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub y_offset: usize,
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
        Self::new(crop_border.width, crop_border.height)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct CropProcessorOptions {
    pub crop_border: CropBorder,
}

pub struct CropProcessor {
    options: CropProcessorOptions,
}

impl CropProcessor {
    pub fn new(options: CropProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for CropProcessor {
    fn name(&self) -> &'static str {
        "crop"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let mut new_image = FastImage::empty(self.options.crop_border.into());

        new_image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                let new_pixel = image.get_image_pixel(coord + self.options.crop_border.offset());
                *pixel = new_pixel;
            },
            Some(progress),
        );

        Ok(new_image)
    }
}

use picturify_core::core::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::geometry::angle::Angle;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

use crate::common::execution::Processor;

pub struct RotateFlexibleProcessorOptions {
    pub angle: Angle,
}

pub struct RotateFlexibleProcessor {
    options: RotateFlexibleProcessorOptions,
}

impl RotateFlexibleProcessor {
    pub fn new(options: RotateFlexibleProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for RotateFlexibleProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> ProcessingPicturifyResult<FastImage> {
        let (width, height) = image.size().into();
        let size = Size::new(width, height);
        let new_size = size.rotate(self.options.angle);
        let (new_width, new_height): (usize, usize) = new_size.into();

        let mut new_image = FastImage::empty(new_size);
        let rotation_origin = (width / 2, height / 2).into();
        let mut shift: Coord = (new_width - width, new_height - height).into();
        shift /= 2;

        let angle = -self.options.angle;

        new_image.par_apply_fn_to_image_pixel(
            |pixel, mut coord| {
                coord -= shift;
                coord = coord.rotate(angle, rotation_origin);
                if coord.in_bounds(size) {
                    *pixel = image.get_image_pixel(coord);
                }
            },
            Some(progress),
        );

        Ok(new_image)
    }
}

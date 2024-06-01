use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::angle::Angle;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

pub struct RotateFlexibleProcessorOptions {
    pub angle: Angle,
}

impl Default for RotateFlexibleProcessorOptions {
    fn default() -> Self {
        RotateFlexibleProcessorOptions {
            angle: Angle::from_degrees(0.0),
        }
    }
}

pub struct RotateFlexibleProcessor {
    options: RotateFlexibleProcessorOptions,
}

impl RotateFlexibleProcessor {
    pub fn new() -> Self {
        RotateFlexibleProcessor {
            options: RotateFlexibleProcessorOptions::default(),
        }
    }
}

impl WithOptions<RotateFlexibleProcessorOptions> for RotateFlexibleProcessor {
    fn with_options(mut self, options: RotateFlexibleProcessorOptions) -> Self {
        self.options = options;
        self
    }
}

impl Processor for RotateFlexibleProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let (width, height) = image.size().into();
        let size = Size::new(width, height);
        let new_size = size.rotate(self.options.angle);
        let (new_width, new_height): (usize, usize) = new_size.into();

        let mut new_image = FastImage::empty(new_size);
        let rotation_origin = (width / 2, height / 2).into();
        let mut shift = (new_width - width, new_height - height).into();
        shift = shift / 2;

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

        new_image
    }
}

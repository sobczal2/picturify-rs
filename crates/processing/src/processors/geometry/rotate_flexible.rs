use picturify_core::common::angle::Angle;
use picturify_core::common::cord::Cord;
use picturify_core::common::size::Size;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use crate::common::execution::{Processor, WithOptions};

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
        let width = image.get_width();
        let height = image.get_height();
        
        let size = Size::new(width, height);
        let new_size = size.rotate(self.options.angle);
        
        let mut new_image = FastImage::empty(new_size.width, new_size.height);
        let rotation_origin = Cord::new(new_size.width / 2, new_size.height / 2);
        let shift = Cord::from_i32(
            (new_size.width as i32 - width as i32) / 2,
            (new_size.height as i32 - height as i32) / 2,
        );
        
        let angle = -self.options.angle;
        
        new_image.par_apply_fn_to_image_pixel(
            |pixel, x, y| {
                let mut new_cord = Cord::new(x, y);
                new_cord = new_cord.rotate(angle, rotation_origin);
                new_cord -= shift;
                if new_cord.in_bounds(width, height) {
                    *pixel = image.get_image_pixel(new_cord.get_x(), new_cord.get_y());
                }
            },
            Some(progress)
        );
        
        new_image
    }
}
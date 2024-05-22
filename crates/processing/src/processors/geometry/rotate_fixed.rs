use crate::common::execution::{Processor, WithOptions};
use picturify_core::common::angle::Angle;
use picturify_core::error::processing::ProcessingError;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

#[derive(Copy, Clone)]
pub enum RotateFixedStrategy {
    Deg90,
    Deg180,
    Deg270,
}

impl RotateFixedStrategy {
    fn get_new_dimensions(&self, width: usize, height: usize) -> (usize, usize) {
        match self {
            RotateFixedStrategy::Deg90 => (height, width),
            RotateFixedStrategy::Deg180 => (width, height),
            RotateFixedStrategy::Deg270 => (height, width),
        }
    }

    fn rotate_pixel(&self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        match self {
            RotateFixedStrategy::Deg90 => (y, width - x - 1),
            RotateFixedStrategy::Deg180 => (width - x - 1, height - y - 1),
            RotateFixedStrategy::Deg270 => (height - y - 1, x),
        }
    }
}

impl TryFrom<Angle> for RotateFixedStrategy {
    type Error = ProcessingError;

    fn try_from(value: Angle) -> Result<Self, Self::Error> {
        let angle = value.to_degrees();
        match angle {
            90.0 => Ok(RotateFixedStrategy::Deg90),
            180.0 => Ok(RotateFixedStrategy::Deg180),
            270.0 => Ok(RotateFixedStrategy::Deg270),
            _ => Err(ProcessingError::InvalidAngle),
        }
    }
}

#[derive(Copy, Clone)]
pub struct RoteteFixedProcessorOptions {
    pub strategy: RotateFixedStrategy,
}

impl Default for RoteteFixedProcessorOptions {
    fn default() -> Self {
        RoteteFixedProcessorOptions {
            strategy: RotateFixedStrategy::Deg90,
        }
    }
}

pub struct RotateFixedProcessor {
    options: RoteteFixedProcessorOptions,
}

impl RotateFixedProcessor {
    pub fn new() -> Self {
        RotateFixedProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<RoteteFixedProcessorOptions> for RotateFixedProcessor {
    fn with_options(mut self, options: RoteteFixedProcessorOptions) -> Self {
        self.options = options;
        self
    }
}

impl Processor for RotateFixedProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let width = image.get_width();
        let height = image.get_height();
        let (new_width, new_height) = self.options.strategy.get_new_dimensions(width, height);

        let mut new_image = FastImage::empty(new_width, new_height);

        new_image.par_apply_fn_to_image_pixel(
            |pixel, x, y| {
                let (new_x, new_y) = self
                    .options
                    .strategy
                    .rotate_pixel(x, y, new_width, new_height);
                *pixel = image.get_image_pixel(new_x, new_y)
            },
            Some(progress),
        );

        new_image
    }
}

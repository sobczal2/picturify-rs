use crate::common::execution::{Processor, WithOptions};
use picturify_core::error::processing::ProcessingError;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::angle::Angle;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

#[derive(Copy, Clone)]
pub enum RotateFixedStrategy {
    Deg90,
    Deg180,
    Deg270,
}

impl RotateFixedStrategy {
    fn get_new_size(&self, size: Size) -> Size {
        match self {
            RotateFixedStrategy::Deg90 => size.rotate_90(),
            RotateFixedStrategy::Deg180 => size,
            RotateFixedStrategy::Deg270 => size.rotate_90(),
        }
    }

    fn rotate_pixel(&self, coord: Coord, size: Size) -> Coord {
        let (x, y): (i32, i32) = coord.into();
        let (width, height): (i32, i32) = size.into();
        match self {
            RotateFixedStrategy::Deg90 => (y, width - x - 1).into(),
            RotateFixedStrategy::Deg180 => (width - x - 1, height - y - 1).into(),
            RotateFixedStrategy::Deg270 => (height - y - 1, x).into(),
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
        let new_size = self.options.strategy.get_new_size(image.size());

        let mut new_image = FastImage::empty(new_size);

        new_image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                let new_coord = self
                    .options
                    .strategy
                    .rotate_pixel(coord, new_size);
                *pixel = image.get_image_pixel(new_coord)
            },
            Some(progress),
        );

        new_image
    }
}

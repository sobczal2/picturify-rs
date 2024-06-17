use picturify_core::core::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::{ProcessingPicturifyError, ProcessingPicturifyResult};
use picturify_core::geometry::angle::Angle;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::threading::progress::Progress;

use crate::common::execution::Processor;

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
    type Error = ProcessingPicturifyError;

    fn try_from(value: Angle) -> Result<Self, Self::Error> {
        let angle = value.to_degrees();
        if (angle - 90f32).abs() < 0.1 {
            Ok(RotateFixedStrategy::Deg90)
        } else if (angle - 180f32).abs() < 0.1 {
            Ok(RotateFixedStrategy::Deg180)
        } else if (angle - 270f32).abs() < 0.1 {
            Ok(RotateFixedStrategy::Deg270)
        } else {
            Err(ProcessingPicturifyError::InvalidAngle)
        }
    }
}

#[derive(Copy, Clone)]
pub struct RoteteFixedProcessorOptions {
    pub strategy: RotateFixedStrategy,
}

pub struct RotateFixedProcessor {
    options: RoteteFixedProcessorOptions,
}

impl RotateFixedProcessor {
    pub fn new(options: RoteteFixedProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for RotateFixedProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let new_size = self.options.strategy.get_new_size(image.size());

        let mut new_image = FastImage::empty(new_size);

        new_image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                let new_coord = self.options.strategy.rotate_pixel(coord, new_size);
                *pixel = image.get_image_pixel(new_coord)
            },
            Some(progress),
        );

        Ok(new_image)
    }
}

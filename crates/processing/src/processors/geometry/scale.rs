use crate::common::processors::CpuProcessor;
use picturify_core::core::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::image::Rgba;
use picturify_core::threading::progress::Progress;

#[derive(Clone, Copy)]
pub enum ScaleStrategy {
    NearestNeighbor,
    Bilinear,
}

impl ScaleStrategy {
    fn get_pixel(&self, read_image: &FastImage, coord: Coord, new_size: Size) -> Rgba<u8> {
        match self {
            Self::NearestNeighbor => {
                let (old_width, old_height): (usize, usize) = read_image.size().into();
                let (new_width, new_height): (usize, usize) = new_size.into();
                let (x, y): (usize, usize) = coord.into();
                let x = x * old_width / new_width;
                let y = y * old_height / new_height;
                read_image.get_image_pixel((x, y).into())
            }
            ScaleStrategy::Bilinear => Rgba([0, 0, 0, 0]),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ScaleProcessorOptions {
    pub size: Size,
    pub strategy: ScaleStrategy,
}

pub struct ScaleProcessor {
    options: ScaleProcessorOptions,
}

impl ScaleProcessor {
    pub fn new(options: ScaleProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for ScaleProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let new_size = self.options.size;
        let strategy = self.options.strategy;

        let mut new_image = FastImage::empty(new_size);

        new_image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                *pixel = strategy.get_pixel(&image, coord, new_size);
            },
            Some(progress),
        );

        Ok(new_image)
    }
}

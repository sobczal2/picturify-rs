use crate::common::processors::CpuProcessor;
use picturify_core::core::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::pixel::math::PixelMath;
use picturify_core::threading::progress::Progress;

pub struct AddProcessorOptions {
    pub second_image: FastImage,
}

pub struct AddProcessor {
    options: AddProcessorOptions,
}

impl AddProcessor {
    pub fn new(options: AddProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for AddProcessor {
    fn name(&self) -> &'static str {
        "add"
    }
    fn process(
        &self,
        mut image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let second_image = &self.options.second_image;

        image.par_apply_fn_to_image_pixel(
            |pixel, coord| {
                let second_pixel = second_image.get_image_pixel(coord);
                pixel.add_assign(second_pixel);
            },
            Some(progress),
        );
        Ok(image)
    }
}

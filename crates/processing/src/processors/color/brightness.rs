use serde::{Deserialize, Serialize};
use picturify_core::core::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::palette::Hsla;
use picturify_core::threading::progress::Progress;

use crate::common::processors::CpuProcessor;

#[derive(Serialize, Deserialize)]
pub struct BrightnessProcessorOptions {
    pub factor: f32,
}

pub struct BrightnessProcessor {
    options: BrightnessProcessorOptions,
}

impl BrightnessProcessor {
    pub fn new(options: BrightnessProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for BrightnessProcessor {
    fn name(&self) -> &'static str {
        "brightness"
    }
    fn process(
        &self,
        mut image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        image.par_apply_fn_to_pixel(
            |mut pixel: Hsla, _coord| {
                pixel.lightness = (pixel.lightness * self.options.factor).clamp(0.0, 1.0);
                pixel
            },
            Some(progress),
        );

        Ok(image)
    }
}

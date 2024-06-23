use serde::{Deserialize, Serialize};
use picturify_core::core::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::pixel::traits::RgbaU8Pixel;
use picturify_core::threading::progress::Progress;

use crate::common::processors::CpuProcessor;

#[derive(Serialize, Deserialize)]
pub struct QuantizationProcessorOptions {
    pub levels: u8,
    pub use_fast_approximation: bool,
}

pub struct QuantizationProcessor {
    options: QuantizationProcessorOptions,
}

impl QuantizationProcessor {
    pub fn new(options: QuantizationProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for QuantizationProcessor {
    fn name(&self) -> &'static str {
        "quantization"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        if self.options.use_fast_approximation {
            self.process_fast(image, progress)
        } else {
            self.process_slow(image, progress)
        }
    }
}

impl QuantizationProcessor {
    fn process_fast(
        &self,
        mut image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let quantization_map = QuantizationLevelMap::new(self.options.levels);
        image.par_apply_fn_to_image_pixel(
            |pixel, _coord| {
                let red = pixel.red_u8();
                let green = pixel.green_u8();
                let blue = pixel.blue_u8();

                let red = quantization_map.quantize_channel(red);
                let green = quantization_map.quantize_channel(green);
                let blue = quantization_map.quantize_channel(blue);

                pixel.set_red_u8(red);
                pixel.set_green_u8(green);
                pixel.set_blue_u8(blue);
            },
            Some(progress),
        );
        Ok(image)
    }

    fn process_slow(
        &self,
        mut image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let quantization_map = QuantizationLevelMap::new(self.options.levels);
        image.par_apply_fn_to_lin_srgba(
            |mut pixel, _coord| {
                let red = pixel.red_u8();
                let green = pixel.green_u8();
                let blue = pixel.blue_u8();

                let red = quantization_map.quantize_channel(red);
                let green = quantization_map.quantize_channel(green);
                let blue = quantization_map.quantize_channel(blue);

                pixel.set_red_u8(red);
                pixel.set_green_u8(green);
                pixel.set_blue_u8(blue);
                pixel
            },
            Some(progress),
        );
        Ok(image)
    }
}

struct QuantizationLevelMap {
    levels: [u8; 256],
}

impl QuantizationLevelMap {
    fn new(levels: u8) -> Self {
        let level_size = (255 / (levels as u16 + 1)) as u32;
        let mut levels = [0; 256];
        let mut current_level = 0;
        for i in 0..256 {
            if i >= (current_level * level_size + level_size / 2) {
                current_level += 1;
            }
            levels[i as usize] = (current_level * level_size) as u8;
        }
        Self { levels }
    }

    #[inline(always)]
    fn quantize_channel(&self, channel_value: u8) -> u8 {
        self.levels[channel_value as usize]
    }
}

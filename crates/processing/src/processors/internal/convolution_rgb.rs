use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::palette::LinSrgba;
use picturify_core::threading::progress::Progress;
use std::sync::{Arc, RwLock};

use crate::common::execution::{Processor, WithOptions};

pub struct ConvolutionRgbProcessorOptions {
    pub kernel: Vec<f32>,
    pub kernel_width: usize,
    pub kernel_height: usize,
    pub kernel_divisor: f32,
    pub kernel_offset: f32,
    pub use_fast_approximation: bool,
}

impl Default for ConvolutionRgbProcessorOptions {
    fn default() -> Self {
        ConvolutionRgbProcessorOptions {
            kernel: vec![0.0; 9],
            kernel_width: 3,
            kernel_height: 3,
            kernel_divisor: 1.0,
            kernel_offset: 0.0,
            use_fast_approximation: true,
        }
    }
}

pub struct ConvolutionRgbProcessor {
    options: ConvolutionRgbProcessorOptions,
}

impl ConvolutionRgbProcessor {
    pub fn new() -> Self {
        ConvolutionRgbProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<ConvolutionRgbProcessorOptions> for ConvolutionRgbProcessor {
    fn with_options(self, options: ConvolutionRgbProcessorOptions) -> Self {
        ConvolutionRgbProcessor { options }
    }
}

impl Processor for ConvolutionRgbProcessor {
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let mut new_image = fast_image.clone();

        if self.options.use_fast_approximation {
            new_image.par_apply_fn_to_image_pixel(
                |pixel, x, y| {
                    if x < self.options.kernel_width / 2
                        || x >= width - self.options.kernel_width / 2
                        || y < self.options.kernel_height / 2
                        || y >= height - self.options.kernel_height / 2
                    {
                        return;
                    }

                    let mut result_red_f32 = 0f32;
                    let mut result_green_f32 = 0f32;
                    let mut result_blue_f32 = 0f32;

                    for i in 0..self.options.kernel_width {
                        for j in 0..self.options.kernel_height {
                            let kernel_value =
                                self.options.kernel[j * self.options.kernel_width + i];
                            let image_pixel = fast_image.get_image_pixel(
                                x + i - self.options.kernel_width / 2,
                                y + j - self.options.kernel_height / 2,
                            );
                            result_red_f32 += image_pixel.0[0] as f32 / 255.0 * kernel_value;
                            result_green_f32 += image_pixel.0[1] as f32 / 255.0 * kernel_value;
                            result_blue_f32 += image_pixel.0[2] as f32 / 255.0 * kernel_value;
                        }
                    }

                    result_red_f32 =
                        result_red_f32 / self.options.kernel_divisor + self.options.kernel_offset;
                    result_green_f32 =
                        result_green_f32 / self.options.kernel_divisor + self.options.kernel_offset;
                    result_blue_f32 =
                        result_blue_f32 / self.options.kernel_divisor + self.options.kernel_offset;

                    let result_red = (result_red_f32 * 255.0).clamp(0.0, 255.0).round() as u8;
                    let result_green = (result_green_f32 * 255.0).clamp(0.0, 255.0).round() as u8;
                    let result_blue = (result_blue_f32 * 255.0).clamp(0.0, 255.0).round() as u8;

                    pixel.0[0] = result_red;
                    pixel.0[1] = result_green;
                    pixel.0[2] = result_blue;
                },
                Some(progress),
            );
        } else {
            new_image.par_apply_fn_to_lin_srgba(
                |pixel, x, y| {
                    if x < self.options.kernel_width / 2
                        || x >= width - self.options.kernel_width / 2
                        || y < self.options.kernel_height / 2
                        || y >= height - self.options.kernel_height / 2
                    {
                        return pixel;
                    }

                    let mut new_pixel = LinSrgba::new(0.0, 0.0, 0.0, pixel.alpha);

                    for i in 0..self.options.kernel_width {
                        for j in 0..self.options.kernel_height {
                            let kernel_value =
                                self.options.kernel[j * self.options.kernel_width + i];
                            let image_pixel = fast_image.get_lin_srgba_pixel(
                                x + i - self.options.kernel_width / 2,
                                y + j - self.options.kernel_height / 2,
                            );
                            let image_pixel = LinSrgba::from(image_pixel);
                            new_pixel += image_pixel * kernel_value;
                        }
                    }

                    new_pixel =
                        new_pixel / self.options.kernel_divisor + self.options.kernel_offset;
                    new_pixel
                },
                Some(progress),
            );
        }

        new_image
    }
}

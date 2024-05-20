use std::sync::{Arc, RwLock};

use picturify_core::fast_image::apply_fn_to_pixels::{ApplyFnToImagePixels, Offset};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor, WithOptions};
use crate::helpers::functions::gaussian_1d;
use crate::helpers::kernels::ConvolutionKernel;

pub struct BilateralProcessorOptions {
    pub radius: usize,
    pub sigma_spatial: f32,
    pub sigma_intensity: f32,
    pub use_fast_approximation: bool,
}

impl Default for BilateralProcessorOptions {
    fn default() -> BilateralProcessorOptions {
        BilateralProcessorOptions {
            radius: 3,
            sigma_spatial: 1.0,
            sigma_intensity: 1.0,
            use_fast_approximation: false,
        }
    }
}

pub struct BilateralProcessor {
    options: BilateralProcessorOptions,
}

impl BilateralProcessor {
    pub fn new() -> Self {
        BilateralProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<BilateralProcessorOptions> for BilateralProcessor {
    fn with_options(self, options: BilateralProcessorOptions) -> Self {
        BilateralProcessor { options }
    }
}

// TODO fix this
impl Processor for BilateralProcessor {
    fn process(&self, image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let height = image.get_height();
        let width = image.get_width();

        let spatial_kernel =
            ConvolutionKernel::new_gaussian(self.options.radius, self.options.sigma_spatial);

        let mut new_image = image.clone();

        let offset = Offset {
            skip_rows: self.options.radius,
            take_rows: height - 2 * self.options.radius,
            skip_columns: self.options.radius,
            take_columns: width - 2 * self.options.radius,
        };

        let half_kernel_size = self.options.radius as i32;

        if self.options.use_fast_approximation {
            new_image.par_apply_fn_to_image_pixel_with_offset(
                |pixel, x, y| {
                    let mut wp_total = 0.0f32;
                    let red = pixel.0[0] as f32 / 255.0f32;
                    let green = pixel.0[1] as f32 / 255.0f32;
                    let blue = pixel.0[2] as f32 / 255.0f32;
                    let mut output_values = [0.0f32; 3];

                    for k in -half_kernel_size..=half_kernel_size {
                        for l in -half_kernel_size..=half_kernel_size {
                            let ni = y + k as usize;
                            let nj = x + l as usize;

                            let spatial_weight = spatial_kernel.get(
                                (l + half_kernel_size) as usize,
                                (k + half_kernel_size) as usize,
                            );

                            let current_inner_pixel = image.get_image_pixel(nj, ni);
                            let current_inner_red = current_inner_pixel.0[0] as f32 / 255.0f32;
                            let current_inner_green = current_inner_pixel.0[1] as f32 / 255.0f32;
                            let current_inner_blue = current_inner_pixel.0[2] as f32 / 255.0f32;

                            let color_distance = (red - current_inner_red).powf(2f32)
                                + (green - current_inner_green).powf(2f32)
                                + (blue - current_inner_blue).powf(2f32);
                            let intensity_weight =
                                gaussian_1d(color_distance, self.options.sigma_intensity);

                            let weight = spatial_weight * intensity_weight;

                            wp_total += weight;

                            output_values[0] += weight * current_inner_red;
                            output_values[1] += weight * current_inner_green;
                            output_values[2] += weight * current_inner_blue;
                        }
                    }

                    let red = ((output_values[0] / wp_total) * 255.0).clamp(0.0, 255.0) as u8;
                    let green = ((output_values[1] / wp_total) * 255.0).clamp(0.0, 255.0) as u8;
                    let blue = ((output_values[2] / wp_total) * 255.0).clamp(0.0, 255.0) as u8;

                    pixel.0[0] = red;
                    pixel.0[1] = green;
                    pixel.0[2] = blue;
                },
                Some(progress),
                offset,
            );
        } else {
        }

        new_image
    }
}
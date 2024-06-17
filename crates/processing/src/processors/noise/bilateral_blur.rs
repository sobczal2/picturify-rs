use picturify_core::core::apply_fn_to_pixels::{ApplyFnToImagePixels, Offset};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

use crate::common::functions::gaussian_1d;
use crate::common::kernels::convolution::ConvolutionKernel;
use crate::common::processors::CpuProcessor;

pub struct BilateralBlurProcessorOptions {
    pub radius: usize,
    pub sigma_spatial: f32,
    pub sigma_intensity: f32,
    pub use_fast_approximation: bool,
}

pub struct BilateralBlurProcessor {
    options: BilateralBlurProcessorOptions,
}

impl BilateralBlurProcessor {
    pub fn new(options: BilateralBlurProcessorOptions) -> Self {
        Self { options }
    }
}

// TODO fix this
impl CpuProcessor for BilateralBlurProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let (width, height): (usize, usize) = image.size().into();

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
                |pixel, coord| {
                    let mut wp_total = 0.0f32;
                    let red = pixel.0[0] as f32 / 255.0f32;
                    let green = pixel.0[1] as f32 / 255.0f32;
                    let blue = pixel.0[2] as f32 / 255.0f32;
                    let mut output_values = [0.0f32; 3];

                    for k in -half_kernel_size..=half_kernel_size {
                        for l in -half_kernel_size..=half_kernel_size {
                            let (x, y): (i32, i32) = coord.into();
                            let ni = y + k;
                            let nj = x + l;

                            let spatial_weifht_coord =
                                (l + half_kernel_size, k + half_kernel_size).into();
                            let spatial_weight = spatial_kernel.get(spatial_weifht_coord);

                            let current_inner_pixel = image.get_image_pixel((nj, ni).into());
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
        }

        Ok(new_image)
    }
}

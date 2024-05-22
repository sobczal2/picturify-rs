use picturify_core::conversions::image_palette_bridge::lin_srgba_to_rgba;
use picturify_core::fast_image::apply_fn_to_pixels::{ApplyFnToImagePixels, Offset};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

use crate::common::execution::{Processor, WithOptions};
use crate::helpers::kernels::ConvolutionKernel;

pub struct ConvolutionRgbProcessorOptions {
    pub kernel: ConvolutionKernel,
    pub use_fast_approximation: bool,
}

impl Default for ConvolutionRgbProcessorOptions {
    fn default() -> Self {
        ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new(vec![vec![1.0]]),
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
    fn process(&self, image: FastImage, mut progress: Progress) -> FastImage {
        let (width, height): (usize, usize) = image.size().into();

        let mut new_image = image.clone();
        
        let (kernel_width, kernel_height): (usize, usize) = self.options.kernel.size().into();
        let half_kernel_width = kernel_width / 2;
        let half_kernel_height = kernel_height / 2;

        progress.setup(height - 2 * half_kernel_height);
        let offset = Offset {
            skip_rows: half_kernel_height,
            take_rows: height - 2 * half_kernel_height,
            skip_columns: half_kernel_width,
            take_columns: width - 2 * half_kernel_width,
        };

        new_image.par_apply_fn_to_image_pixel_with_offset(
            |pixel, coord| {
                *pixel = match self.options.use_fast_approximation {
                    true => self.options.kernel.convolve_rgb_fast(&image, coord),
                    false => {
                        let lin_srgba = self.options.kernel.convolve_rgb_slow(&image, coord);
                        lin_srgba_to_rgba(lin_srgba)
                    }
                }
            },
            Some(progress.clone()),
            offset,
        );

        new_image
    }
}

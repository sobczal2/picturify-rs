use crate::common::execution::{Processor, WithOptions};
use crate::helpers::kernels::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct GaussianBlurProcessorOptions {
    pub radius: usize,
    pub sigma: f32,
    pub use_fast_approximation: bool,
}

impl Default for GaussianBlurProcessorOptions {
    fn default() -> GaussianBlurProcessorOptions {
        GaussianBlurProcessorOptions {
            radius: 3,
            sigma: 1.0,
            use_fast_approximation: true,
        }
    }
}

pub struct GaussianBlurProcessor {
    options: GaussianBlurProcessorOptions,
}

impl GaussianBlurProcessor {
    pub fn new() -> Self {
        GaussianBlurProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<GaussianBlurProcessorOptions> for GaussianBlurProcessor {
    fn with_options(self, options: GaussianBlurProcessorOptions) -> Self {
        GaussianBlurProcessor { options }
    }
}

impl Processor for GaussianBlurProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let radius = self.options.radius;
        let sigma = self.options.sigma;

        let processor =
            ConvolutionRgbProcessor::new().with_options(ConvolutionRgbProcessorOptions {
                kernel: ConvolutionKernel::new_gaussian(radius, sigma),
                use_fast_approximation: self.options.use_fast_approximation,
            });

        processor.process(image, progress)
    }
}

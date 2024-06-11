use crate::common::execution::{Processor, WithOptions};
use crate::helpers::kernels::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct MeanBlurProcessorOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

impl Default for MeanBlurProcessorOptions {
    fn default() -> MeanBlurProcessorOptions {
        MeanBlurProcessorOptions {
            radius: 3,
            use_fast_approximation: true,
        }
    }
}

pub struct MeanBlurProcessor {
    options: MeanBlurProcessorOptions,
}

impl MeanBlurProcessor {
    pub fn new() -> Self {
        MeanBlurProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<MeanBlurProcessorOptions> for MeanBlurProcessor {
    fn with_options(self, options: MeanBlurProcessorOptions) -> Self {
        MeanBlurProcessor { options }
    }
}

impl Processor for MeanBlurProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let radius = self.options.radius;

        let processor =
            ConvolutionRgbProcessor::new().with_options(ConvolutionRgbProcessorOptions {
                kernel: ConvolutionKernel::new_mean(radius),
                use_fast_approximation: self.options.use_fast_approximation,
            });
        processor.process(image, progress)
    }
}

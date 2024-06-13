use crate::common::execution::Processor;
use crate::helpers::kernels::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct MeanBlurProcessorOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

pub struct MeanBlurProcessor {
    options: MeanBlurProcessorOptions,
}

impl MeanBlurProcessor {
    pub fn new(options: MeanBlurProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for MeanBlurProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let radius = self.options.radius;

        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_mean(radius),
            use_fast_approximation: self.options.use_fast_approximation,
        });
        processor.process(image, progress)
    }
}

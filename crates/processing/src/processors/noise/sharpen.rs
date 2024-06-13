use crate::common::execution::Processor;
use crate::helpers::kernels::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct SharpenProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct SharpenProcessor {
    options: SharpenProcessorOptions,
}

impl SharpenProcessor {
    pub fn new(options: SharpenProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for SharpenProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_sharpen(),
            use_fast_approximation: self.options.use_fast_approximation,
        });
        processor.process(image, progress)
    }
}

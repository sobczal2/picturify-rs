use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

use crate::common::processors::CpuProcessor;
use crate::common::kernels::convolution::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};

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

impl CpuProcessor for MeanBlurProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let radius = self.options.radius;

        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_mean(radius),
            use_fast_approximation: self.options.use_fast_approximation,
        });
        processor.process(image, progress)
    }
}

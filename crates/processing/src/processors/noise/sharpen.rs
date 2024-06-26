use serde::{Deserialize, Serialize};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

use crate::common::kernels::convolution::ConvolutionKernel;
use crate::common::processors::CpuProcessor;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};

#[derive(Serialize, Deserialize, Clone)]
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

impl CpuProcessor for SharpenProcessor {
    fn name(&self) -> &'static str {
        "sharpen"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_sharpen(),
            use_fast_approximation: self.options.use_fast_approximation,
        });
        processor.process(image, progress)
    }
}

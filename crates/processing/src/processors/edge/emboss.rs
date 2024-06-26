use serde::{Deserialize, Serialize};
use crate::common::kernels::convolution::ConvolutionKernel;
use crate::common::processors::CpuProcessor;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

#[derive(Serialize, Deserialize)]
pub struct EmbossProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct EmbossProcessor {
    options: EmbossProcessorOptions,
}

impl EmbossProcessor {
    pub fn new(options: EmbossProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for EmbossProcessor {
    fn name(&self) -> &'static str {
        "emboss"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_emboss(),
            use_fast_approximation: self.options.use_fast_approximation,
        });
        processor.process(image, progress)
    }
}

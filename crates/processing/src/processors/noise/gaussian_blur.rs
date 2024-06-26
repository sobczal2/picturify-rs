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
pub struct GaussianBlurProcessorOptions {
    pub radius: usize,
    pub sigma: f32,
    pub use_fast_approximation: bool,
}

pub struct GaussianBlurProcessor {
    options: GaussianBlurProcessorOptions,
}

impl GaussianBlurProcessor {
    pub fn new(options: GaussianBlurProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for GaussianBlurProcessor {
    fn name(&self) -> &'static str {
        "gaussian-blur"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let radius = self.options.radius;
        let sigma = self.options.sigma;

        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_gaussian(radius, sigma),
            use_fast_approximation: self.options.use_fast_approximation,
        });

        processor.process(image, progress)
    }
}

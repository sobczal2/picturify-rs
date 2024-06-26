use serde::{Deserialize, Serialize};
use crate::common::kernels::sobel::SobelKernels;
use crate::common::processors::CpuProcessor;
use crate::processors::internal::gradient_based_rgb::{
    GradientBasedRgbProcessor, GradientBasedRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

#[derive(Serialize, Deserialize)]
pub struct SobelRgbProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct SobelRgbProcessor {
    options: SobelRgbProcessorOptions,
}

impl SobelRgbProcessor {
    pub fn new(options: SobelRgbProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for SobelRgbProcessor {
    fn name(&self) -> &'static str {
        "sobel-rgb"
    }
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let inner_processor_options = GradientBasedRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            xy_kernels: SobelKernels::create().unwrap(),
        };
        let inner_processor = GradientBasedRgbProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}

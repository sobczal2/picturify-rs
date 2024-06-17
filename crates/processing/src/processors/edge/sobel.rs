use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

use crate::common::processors::CpuProcessor;
use crate::common::kernels::sobel::SobelKernels;
use crate::processors::edge::gradient_based::{
    GradientBasedProcessor, GradientBasedProcessorOptions,
};

pub struct SobelProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct SobelProcessor {
    options: SobelProcessorOptions,
}

impl SobelProcessor {
    pub fn new(options: SobelProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for SobelProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let inner_processor_options = GradientBasedProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            xy_kernels: SobelKernels::create().unwrap(),
        };
        let inner_processor = GradientBasedProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}

use crate::common::kernels::prewitt::PrewittKernels;
use crate::common::processors::CpuProcessor;
use crate::processors::internal::gradient_based_rgb::{
    GradientBasedRgbProcessor, GradientBasedRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

pub struct PrewittRgbProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct PrewittRgbProcessor {
    options: PrewittRgbProcessorOptions,
}

impl PrewittRgbProcessor {
    pub fn new(options: PrewittRgbProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for PrewittRgbProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let inner_processor_options = GradientBasedRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            xy_kernels: PrewittKernels::create().unwrap(),
        };
        let inner_processor = GradientBasedRgbProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}

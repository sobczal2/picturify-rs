use crate::common::kernels::prewitt::PrewittKernels;
use crate::common::processors::CpuProcessor;
use crate::processors::edge::gradient_based::{
    GradientBasedProcessor, GradientBasedProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

pub struct PrewittProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct PrewittProcessor {
    options: PrewittProcessorOptions,
}

impl PrewittProcessor {
    pub fn new(options: PrewittProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for PrewittProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let inner_processor_options = GradientBasedProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            xy_kernels: PrewittKernels::create().unwrap(),
        };
        let inner_processor = GradientBasedProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}

use crate::common::execution::Processor;
use crate::processors::edge::gradient_based_rgb::{
    GradientBasedRgbProcessor, GradientBasedRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use crate::common::kernels::sobel::SobelKernels;

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

impl Processor for SobelRgbProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let inner_processor_options = GradientBasedRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            xy_kernels: SobelKernels::create().unwrap(),
        };
        let inner_processor = GradientBasedRgbProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}

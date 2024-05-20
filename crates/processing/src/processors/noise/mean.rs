use crate::common::execution::{Processor, WithOptions};
use crate::helpers::kernels::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use std::sync::{Arc, RwLock};

pub struct MeanProcessorOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

impl Default for MeanProcessorOptions {
    fn default() -> MeanProcessorOptions {
        MeanProcessorOptions {
            radius: 3,
            use_fast_approximation: true,
        }
    }
}

pub struct MeanProcessor {
    options: MeanProcessorOptions,
}

impl MeanProcessor {
    pub fn new() -> Self {
        MeanProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<MeanProcessorOptions> for MeanProcessor {
    fn with_options(self, options: MeanProcessorOptions) -> Self {
        MeanProcessor { options }
    }
}

impl Processor for MeanProcessor {
    fn process(&self, image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let radius = self.options.radius;

        let processor =
            ConvolutionRgbProcessor::new().with_options(ConvolutionRgbProcessorOptions {
                kernel: ConvolutionKernel::new_mean(radius),
                use_fast_approximation: self.options.use_fast_approximation,
            });
        processor.process(image, progress)
    }
}

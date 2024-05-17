use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use crate::common::execution::{Processor, WithOptions};
use crate::processors::internal::convolution_rgb::{ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions};

pub struct SharpenProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for SharpenProcessorOptions {
    fn default() -> SharpenProcessorOptions {
        SharpenProcessorOptions {
            use_fast_approximation: true,
        }
    }
}

pub struct SharpenProcessor {
    options: SharpenProcessorOptions,
}

impl SharpenProcessor {
    pub fn new() -> Self {
        SharpenProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<SharpenProcessorOptions> for SharpenProcessor {
    fn with_options(self, options: SharpenProcessorOptions) -> Self {
        SharpenProcessor { options }
    }
}

impl Processor for SharpenProcessor {
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {

        let kernel = vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
        let processor =
            ConvolutionRgbProcessor::new().with_options(ConvolutionRgbProcessorOptions {
                kernel,
                kernel_width: 3,
                kernel_height: 3,
                kernel_divisor: 1.0,
                kernel_offset: 0.0,
                use_fast_approximation: self.options.use_fast_approximation,
            });
        processor.process(fast_image, progress)
    }
}
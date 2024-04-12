use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::edge::sobel_rgb::SobelRgbProcessor;
use picturify_processing::processors::geometry::crop::CropProcessor;
use picturify_processing::processors::geometry::enlargement::{EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy};
use crate::pipeline::Pipeline;

pub struct SobelRgbPipelineOptions {
    pub use_fast_approximation: bool,
}

pub struct SobelRgbPipeline {
    options: SobelRgbPipelineOptions,
}

impl SobelRgbPipeline {
    pub fn new(sobel_pipeline_options: SobelRgbPipelineOptions) -> Self {
        Self {
            options: sobel_pipeline_options,
        }
    }

    fn run_fast_approximation(&self, fast_image: FastImage) -> FastImage {
        let sobel_processor = SobelRgbProcessor::new();
        let sobel_image = sobel_processor.process(fast_image);

        sobel_image
    }

    fn run_full(&self, fast_image: FastImage) -> FastImage {
        let radius = 1;
        let original_width = fast_image.get_width();
        let original_height = fast_image.get_height();

        let enlargement_processor = EnlargementProcessor::with_options(
            EnlargementProcessorOptions {
                border: radius,
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
            }
        );

        let enlarged_image = enlargement_processor.process(fast_image);

        let sobel_processor = SobelRgbProcessor::new();
        let sobel_image = sobel_processor.process(enlarged_image);

        let crop_processor = CropProcessor::new(
            radius,
            radius,
            original_width,
            original_height,
        );

        crop_processor.process(sobel_image)
    }
}

impl Pipeline for SobelRgbPipeline {
    fn run(&self, fast_image: FastImage) -> FastImage {
        if self.options.use_fast_approximation {
            self.run_fast_approximation(fast_image)
        } else {
            self.run_full(fast_image)
        }
    }
}
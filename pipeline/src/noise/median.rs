use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::geometry::crop::CropProcessor;
use picturify_processing::processors::geometry::enlargement::{
    EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy,
};
use picturify_processing::processors::noise::median::{MedianProcessor, MedianProcessorOptions};

pub struct MedianPipelineOptions {
    pub radius: usize,
}

pub struct MedianPipeline {
    options: MedianPipelineOptions,
}

impl MedianPipeline {
    pub fn new(median_pipeline_options: MedianPipelineOptions) -> Self {
        Self {
            options: median_pipeline_options,
        }
    }
}

impl Pipeline for MedianPipeline {
    fn run(&self, fast_image: FastImage) -> FastImage {
        let radius = self.options.radius;
        let original_width = fast_image.get_width();
        let original_height = fast_image.get_height();

        let edge_enlargement_processor =
            EnlargementProcessor::with_options(EnlargementProcessorOptions {
                border: radius,
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
            });

        let enlarged_image = edge_enlargement_processor.process(fast_image);

        let median_processor = MedianProcessor::with_options(MedianProcessorOptions { radius });

        let median_image = median_processor.process(enlarged_image);

        let crop_processor = CropProcessor::new(radius, radius, original_width, original_height);

        crop_processor.process(median_image)
    }
}

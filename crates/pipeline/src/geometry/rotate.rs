use picturify_core::core::fast_image::FastImage;
use picturify_core::geometry::angle::Angle;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::geometry::rotate_fixed::{
    RotateFixedProcessor, RotateFixedStrategy, RoteteFixedProcessorOptions,
};
use picturify_processing::processors::geometry::rotate_flexible::{
    RotateFlexibleProcessor, RotateFlexibleProcessorOptions,
};

use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

#[derive(Copy, Clone)]
pub struct RotatePipelineOptions {
    pub angle: Angle,
}

pub struct RotatePipeline {
    options: RotatePipelineOptions,
}

impl RotatePipeline {
    pub fn new(options: RotatePipelineOptions) -> Self {
        Self { options }
    }
}

const ROTATE_PROCESSOR_NAME: &str = "Rotate";

impl Pipeline for RotatePipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage {
        match RotateFixedStrategy::try_from(self.options.angle) {
            Ok(strategy) => self.run_fixed(image, pipeline_progress, strategy),
            Err(_) => self.run_flexible(image, pipeline_progress),
        }
    }
}

impl RotatePipeline {
    fn run_fixed(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
        strategy: RotateFixedStrategy,
    ) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(ROTATE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = RotateFixedProcessor::new(RoteteFixedProcessorOptions { strategy });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());

        pipeline_progress.increment_combined();
        final_image
    }

    fn run_flexible(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
    ) -> FastImage {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(ROTATE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = RotateFlexibleProcessor::new(RotateFlexibleProcessorOptions {
            angle: self.options.angle,
        });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());

        pipeline_progress.increment_combined();
        final_image
    }
}

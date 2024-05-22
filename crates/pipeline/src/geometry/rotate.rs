use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::common::angle::Angle;
use picturify_core::fast_image::FastImage;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::geometry::rotate_fixed::{
    RotateFixedProcessor, RotateFixedStrategy, RoteteFixedProcessorOptions,
};

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
            Err(_) => panic!("Invalid angle"),
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
        let mut pipeline_progress = pipeline_progress.unwrap_or_else(|| PipelineProgress::new());

        pipeline_progress.new_individual(ROTATE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor =
            RotateFixedProcessor::new().with_options(RoteteFixedProcessorOptions { strategy });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress());

        pipeline_progress.increment_combined();
        final_image
    }
}

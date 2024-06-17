use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::quantization::{
    QuantizationProcessor, QuantizationProcessorOptions,
};

use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct QuantizationPipelineOptions {
    pub levels: u8,
    pub fast: bool,
}

pub struct QuantizationPipeline {
    options: QuantizationPipelineOptions,
}

impl QuantizationPipeline {
    pub fn new(options: QuantizationPipelineOptions) -> Self {
        Self { options }
    }
}

const QUANTIZATION_PROCESSOR_NAME: &str = "Quantization";

impl Pipeline for QuantizationPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(QUANTIZATION_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = QuantizationProcessor::new(QuantizationProcessorOptions {
            levels: self.options.levels,
            use_fast_approximation: self.options.fast,
        });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress())?;
        pipeline_progress.increment_combined();
        
        Ok(final_image)
    }
}

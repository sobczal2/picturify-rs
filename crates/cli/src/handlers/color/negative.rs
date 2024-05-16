use std::sync::{Arc, RwLock};
use std::thread::spawn;

use clap::ArgMatches;

use picturify_core::fast_image::FastImage;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;

use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub struct NegativeCommandHandler;

impl NegativeCommandHandler {
    pub fn handle(fast_image: FastImage, _args: ArgMatches) -> FastImage {
        let negative_pipeline = NegativePipeline::new(NegativePipelineOptions {});

        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });
        
        let result_image = negative_pipeline.run(fast_image, pipeline_progress.clone());

        handle.join().expect("Failed to join thread");

        result_image
    }
}

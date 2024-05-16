use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_core::fast_image::FastImage;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::noise::median::{MedianPipeline, MedianPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

pub struct MedianCommandHandler;

impl MedianCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let radius = args.get_one::<usize>("radius").unwrap();
        let negative_pipeline = MedianPipeline::new(MedianPipelineOptions {
            radius: radius.clone(),
        });

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

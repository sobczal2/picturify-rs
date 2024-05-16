use std::sync::{Arc, RwLock};
use std::thread::spawn;
use clap::ArgMatches;
use picturify_core::fast_image::FastImage;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::edge::sobel_rgb::{SobelRgbPipeline, SobelRgbPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub struct SobelRgbCommandHandler;

impl SobelRgbCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let use_fast_approximation = args.get_one::<bool>("use_fast_approximation").unwrap();
        let sobel_rgb_pipeline = SobelRgbPipeline::new(SobelRgbPipelineOptions {
            use_fast_approximation: use_fast_approximation.clone(),
        });

        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = sobel_rgb_pipeline.run(fast_image, pipeline_progress.clone());

        handle.join().expect("Failed to join thread");

        result_image
    }
}
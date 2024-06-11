use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::noise::median_blur::{MedianBlurPipeline, MedianBlurPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use std::thread::spawn;

pub struct MedianBlurCommandHandler;

impl CommandHandler for MedianBlurCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let radius = args.get_one::<usize>(ArgType::Radius.to_id()).unwrap();
        let pipeline = MedianBlurPipeline::new(MedianBlurPipelineOptions {
            fast: fast.clone(),
            radius: radius.clone(),
        });

        let pipeline_progress = PipelineProgress::new();
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = pipeline.run(image, Some(pipeline_progress.clone()));

        handle.join().expect("Failed to join thread");

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
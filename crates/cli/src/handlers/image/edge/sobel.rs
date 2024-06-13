use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::edge::sobel::{SobelPipeline, SobelPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use std::thread::spawn;

pub struct SobelCommandHandler;

impl CommandHandler for SobelCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let rgb = args.get_one::<bool>(ArgType::Rgb.to_id()).unwrap();
        let sobel_pipeline = SobelPipeline::new(SobelPipelineOptions {
            fast: *fast,
            rgb: *rgb,
        });

        let pipeline_progress = PipelineProgress::new();
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = sobel_pipeline.run(image, Some(pipeline_progress.clone()));

        handle.join().expect("Failed to join thread");

        write_image(result_image, args.clone())?;

        Ok(())
    }
}

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::noise::mean::{MeanPipeline, MeanPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

pub struct MeanCommandHandler;

impl CommandHandler for MeanCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let radius = args.get_one::<usize>(ArgType::Radius.to_id()).unwrap();
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let negative_pipeline = MeanPipeline::new(MeanPipelineOptions {
            radius: radius.clone(),
            fast: fast.clone(),
        });

        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = negative_pipeline.run(image, Some(pipeline_progress.clone()));

        handle.join().expect("Failed to join thread");

        write_image(result_image, args.clone())?;

        Ok(())
    }
}

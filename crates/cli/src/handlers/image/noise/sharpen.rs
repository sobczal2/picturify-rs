use std::sync::{Arc, RwLock};
use std::thread::spawn;
use clap::ArgMatches;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::noise::sharpen::{SharpenPipeline, SharpenPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub struct SharpenCommandHandler;

impl CommandHandler for SharpenCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let use_fast_approximation = args.get_one::<bool>(ArgType::UseFastApproximation.to_id()).unwrap();
        let negative_pipeline = SharpenPipeline::new(SharpenPipelineOptions {
            use_fast_approximation: use_fast_approximation.clone(),
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
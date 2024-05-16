use std::sync::{Arc, RwLock};
use std::thread::spawn;

use clap::ArgMatches;

use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};

use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub struct NegativeCommandHandler;

impl CommandHandler for NegativeCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let negative_pipeline = NegativePipeline::new(NegativePipelineOptions {});

        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = negative_pipeline.run(image, pipeline_progress.clone());

        handle.join().expect("Failed to join thread");
        
        write_image(result_image, args.clone())?;
        
        Ok(())
    }
}

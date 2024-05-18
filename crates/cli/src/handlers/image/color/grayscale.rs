use std::sync::{Arc, RwLock};
use std::thread::spawn;
use clap::ArgMatches;
use picturify_pipeline::color::grayscale::{GrayscalePipeline, GrayscalePipelineOptions};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;
use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub struct GrayscaleCommandHandler;

impl CommandHandler for GrayscaleCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let strategy = args.get_one::<GrayscaleStrategy>(ArgType::GrayscaleStrategy.to_id()).unwrap();
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let grayscale_pipeline = GrayscalePipeline::new(GrayscalePipelineOptions {
            strategy: strategy.clone(),
            fast: fast.clone(),
        });
        
        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();
        
        let handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });
        
        let result_image = grayscale_pipeline.run(image, Some(pipeline_progress.clone()));
        
        handle.join().expect("Failed to join thread");
        
        write_image(result_image, args.clone())?;
        
        Ok(())
    }
}
use std::sync::{Arc, RwLock};
use clap::ArgMatches;
use picturify_pipeline::color::brightness::{BrightnessPipeline, BrightnessPipelineOptions};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;
use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};

pub struct BrightnessCommandHandler;

impl CommandHandler for BrightnessCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let factor = args.get_one::<f32>(ArgType::Factor.to_id()).unwrap();
        let brightness_pipeline = BrightnessPipeline::new(BrightnessPipelineOptions {
            factor: *factor,
        });
        
        let pipeline_progress = Arc::new(RwLock::new(PipelineProgress::new()));
        let pipeline_progress_clone = pipeline_progress.clone();
        
        let handle = std::thread::spawn(move || {
            crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline(pipeline_progress_clone);
        });
        
        let result_image = brightness_pipeline.run(image, Some(pipeline_progress.clone()));
        
        handle.join().expect("Failed to join thread");
        
        write_image(result_image, args.clone())?;
        
        Ok(())
    }
}
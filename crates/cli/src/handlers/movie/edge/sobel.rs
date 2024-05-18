use std::sync::{Arc, RwLock};
use std::thread::spawn;
use clap::ArgMatches;
use picturify_movie::movie_pipe::MoviePipe;
use picturify_movie::progress::MovieProgress;
use picturify_pipeline::edge::sobel::{SobelPipeline, SobelPipelineOptions};
use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::progress::movie_progress_bar::run_progress_bar_for_movie;

pub struct SobelCommandHandler;

impl CommandHandler for SobelCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let use_fast_approximation = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let sobel_pipeline = SobelPipeline::new(SobelPipelineOptions {
            use_fast_approximation: use_fast_approximation.clone(),
        });

        let movie_progress = Arc::new(RwLock::new(MovieProgress::new()));
        let movie_progress_clone = movie_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_movie(movie_progress_clone);
        });

        let source = args.get_one::<String>(ArgType::Input.to_id()).unwrap().to_string();
        let destination = args.get_one::<String>(ArgType::Output.to_id()).unwrap().to_string();
        MoviePipe::process(source, destination, Box::new(sobel_pipeline), movie_progress.clone())?;
        handle.join().expect("Failed to join thread");
        
        Ok(())
    }
}
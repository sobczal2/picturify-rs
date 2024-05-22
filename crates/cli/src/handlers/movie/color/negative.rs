use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::progress::movie_progress_bar::run_progress_bar_for_movie;
use clap::ArgMatches;
use picturify_movie::movie_pipe::MoviePipe;
use picturify_movie::progress::MovieProgress;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use std::sync::{Arc, RwLock};
use std::thread::spawn;

pub struct NegativeCommandHandler;

impl CommandHandler for NegativeCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let negative_pipeline = NegativePipeline::new(NegativePipelineOptions {
            fast: *fast,
        });

        let movie_progress = Arc::new(RwLock::new(MovieProgress::new()));
        let movie_progress_clone = movie_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_movie(movie_progress_clone);
        });

        let source = args
            .get_one::<String>(ArgType::Input.to_id())
            .unwrap()
            .to_string();
        let destination = args
            .get_one::<String>(ArgType::Output.to_id())
            .unwrap()
            .to_string();
        MoviePipe::process(
            source,
            destination,
            Box::new(negative_pipeline),
            movie_progress.clone(),
        )?;
        handle.join().expect("Failed to join thread");

        Ok(())
    }
}

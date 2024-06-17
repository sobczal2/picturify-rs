use std::sync::{Arc, RwLock};
use std::thread::spawn;

use clap::ArgMatches;

use picturify_movie::movie_pipe::MoviePipe;
use picturify_movie::progress::MovieProgress;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};

use crate::commands::common::args::common::{FastArg, InputArg, PicturifyArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::progress::movie_progress_bar::run_progress_bar_for_movie;

pub struct NegativeCommandHandler;

impl CommandHandler for NegativeCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let negative_pipeline = NegativePipeline::new(NegativePipelineOptions { fast: *fast });

        let movie_progress = Arc::new(RwLock::new(MovieProgress::default()));
        let movie_progress_clone = movie_progress.clone();

        let handle = spawn(move || {
            run_progress_bar_for_movie(movie_progress_clone);
        });

        let source = args
            .get_one::<String>(InputArg::id())
            .map_to_unknown_error()?;
        let destination = args
            .get_one::<String>(InputArg::id())
            .map_to_unknown_error()?;
        MoviePipe::process(
            source.clone(),
            destination.clone(),
            Box::new(negative_pipeline),
            movie_progress.clone(),
        )?;
        handle.join().expect("Failed to join thread");

        Ok(())
    }
}

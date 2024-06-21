use crate::commands::common::args::common::PicturifyArg;
use clap::ArgMatches;
use picturify_core::rayon::ThreadPoolBuilder;
use picturify_core::{log_debug, log_error, log_warn};
use std::thread::available_parallelism;
use std::time::Instant;

use crate::commands::common::command::Command;
use crate::commands::common::picturify::{PicturifyCommand, PicturifyCpuCountArg};
use crate::common::logging::log_help;
use crate::common::threading::CpuCount;
use crate::error::{CliPicturifyError, CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::completions::CompletionsCommandHandler;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;

pub struct PicturifyCommandHandler;

impl CommandHandler for PicturifyCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let cpu_count = args
            .get_one::<CpuCount>(PicturifyCpuCountArg::id())
            .map_to_unknown_error()?;
        setup_rayon(cpu_count.clone());

        let now = Instant::now();

        let result = match args.subcommand() {
            Some(("image", args)) => {
                ImageCommandHandler::handle(&ImageCommandHandler, args.clone())
            }
            Some(("movie", args)) => {
                MovieCommandHandler::handle(&MovieCommandHandler, args.clone())
            }
            Some(("completions", args)) => {
                CompletionsCommandHandler::handle(&CompletionsCommandHandler, args.clone())
            }
            Some(_) => {
                log_help(&mut PicturifyCommand::create());
                Err(CliPicturifyError::invalid_subcommand())
            }
            None => {
                log_help(&mut PicturifyCommand::create());
                Err(CliPicturifyError::missing_subcommand())
            }
        };

        let elapsed = now.elapsed();
        log_debug!(format!("Command took {}ms", elapsed.as_millis()));

        result
    }
}

fn setup_rayon(cpu_count: CpuCount) {
    let available_parallelism = match available_parallelism() {
        Ok(available_parallelism) => available_parallelism.get(),
        Err(e) => {
            log_error!(e);
            log_warn!("Failed to get available parallelism, using default as max");
            4
        }
    };

    let num_threads = match cpu_count {
        CpuCount::Auto => available_parallelism,
        CpuCount::Count(count) => count,
    };

    if num_threads > available_parallelism {
        log_warn!("Requested thread count is greater than available, may cause performance issues");
    }

    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to create thread pool");

    log_debug!(format!("Using {} threads", num_threads));
}

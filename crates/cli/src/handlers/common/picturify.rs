use std::time::Instant;
use clap::ArgMatches;
use picturify_core::log_debug;

use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::common::logging::log_help;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;

pub struct PicturifyCommandHandler;

impl CommandHandler for PicturifyCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let now = Instant::now();
        
        let result = match args.subcommand() {
            Some(("image", args)) => {
                ImageCommandHandler::handle(&ImageCommandHandler, args.clone())
            }
            Some(("movie", args)) => {
                MovieCommandHandler::handle(&MovieCommandHandler, args.clone())
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

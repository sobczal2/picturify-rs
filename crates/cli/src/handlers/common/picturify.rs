use clap::ArgMatches;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::Command;
use crate::commands::common::picturify::{PicturifyCommand, PicturifyVerbosityArg};
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;

pub struct PicturifyCommandHandler;

impl CommandHandler for PicturifyCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let verbosity = args.get_one::<LevelFilter>(PicturifyVerbosityArg::id()).unwrap();

        TermLogger::init(
            *verbosity,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
            .unwrap();

        match args.subcommand() {
            Some(("image", args)) => ImageCommandHandler::handle(&ImageCommandHandler, args.clone()),
            Some(("movie", args)) => MovieCommandHandler::handle(&MovieCommandHandler, args.clone()),
            _ => {
                PicturifyCommand::get().print_help().unwrap();
                Err(CliPicturifyError::MissingCommand)
            }
        }
    }
}
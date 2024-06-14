use std::collections::HashMap;

use clap::ArgMatches;
use log::warn;

use crate::commands::common::command::{Command, CommandForMovie};
use crate::commands::common::movie::MovieCommand;
use crate::commands::movie::color::negative::NegativeCommand;
use crate::commands::movie::edge::sobel::SobelCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::movie::color::negative::NegativeCommandHandler;
use crate::handlers::movie::edge::sobel::SobelCommandHandler;

pub struct MovieCommandHandler;

impl CommandHandler for MovieCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        warn!("movie subcommand is not yet stable, expect bugs");
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Box<dyn CommandHandler>> = HashMap::new();

                handlers.insert(NegativeCommand::name(), Box::new(NegativeCommandHandler));
                handlers.insert(SobelCommand::name(), Box::new(SobelCommandHandler));

                if let Some(handler) = handlers.get(name) {
                    handler.handle(args.clone())
                } else {
                    Err(CliPicturifyError::InvalidCommand(name.to_string()))
                }
            }
            None => {
                MovieCommand::get().print_help().unwrap();
                Err(CliPicturifyError::MissingSubcommand)
            }
        }
    }
}

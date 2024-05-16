use std::collections::HashMap;
use std::rc::Rc;
use clap::ArgMatches;
use crate::commands::common::command::Command;
use crate::commands::movie::color::negative::NegativeCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::movie::color::negative::NegativeCommandHandler;

pub struct MovieCommandHandler;

impl CommandHandler for MovieCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Rc<dyn Fn(ArgMatches) -> CliPicturifyResult<()>>> = HashMap::new();

                handlers.insert(NegativeCommand::name(), Rc::new(NegativeCommandHandler::handle));

                if let Some(handler) = handlers.get(name) {
                    handler(args.clone())
                } else {
                    Err(CliPicturifyError::InvalidCommand(
                        name.to_string(),
                    ))
                }
            }
            None => {
                Err(CliPicturifyError::MissingCommand)
            }
        }
    }
}
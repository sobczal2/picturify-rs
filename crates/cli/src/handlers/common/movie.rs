use crate::commands::common::command::Command;
use crate::commands::movie::color::negative::NegativeCommand;
use crate::commands::movie::edge::sobel::SobelCommand;
use crate::commands::movie::edge::sobel_rgb::SobelRgbCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::movie::color::negative::NegativeCommandHandler;
use crate::handlers::movie::edge::sobel::SobelCommandHandler;
use crate::handlers::movie::edge::sobel_rgb::SobelRgbCommandHandler;
use clap::ArgMatches;
use std::collections::HashMap;
use std::rc::Rc;

pub struct MovieCommandHandler;

impl CommandHandler for MovieCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Rc<dyn Fn(ArgMatches) -> CliPicturifyResult<()>>> =
                    HashMap::new();

                handlers.insert(
                    NegativeCommand::name(),
                    Rc::new(NegativeCommandHandler::handle),
                );
                handlers.insert(SobelCommand::name(), Rc::new(SobelCommandHandler::handle));
                handlers.insert(
                    SobelRgbCommand::name(),
                    Rc::new(SobelRgbCommandHandler::handle),
                );

                if let Some(handler) = handlers.get(name) {
                    handler(args.clone())
                } else {
                    Err(CliPicturifyError::InvalidCommand(name.to_string()))
                }
            }
            None => Err(CliPicturifyError::MissingCommand),
        }
    }
}

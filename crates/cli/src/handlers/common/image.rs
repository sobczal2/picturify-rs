use std::collections::HashMap;
use std::rc::Rc;
use clap::ArgMatches;
use crate::commands::common::command::Command;
use crate::commands::image::color::negative::NegativeCommand;
use crate::commands::image::color::sepia::SepiaCommand;
use crate::commands::image::common::none::NoneCommand;
use crate::commands::image::edge::sobel::SobelCommand;
use crate::commands::image::edge::sobel_rgb::SobelRgbCommand;
use crate::commands::image::noise::kuwahara::KuwaharaCommand;
use crate::commands::image::noise::mean::MeanCommand;
use crate::commands::image::noise::median::MedianCommand;
use crate::commands::image::noise::sharpen::SharpenCommand;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::image::color::negative::NegativeCommandHandler;
use crate::handlers::image::color::sepia::SepiaCommandHandler;
use crate::handlers::image::common::none::NoneCommandHandler;
use crate::handlers::image::edge::sobel::SobelCommandHandler;
use crate::handlers::image::edge::sobel_rgb::SobelRgbCommandHandler;
use crate::handlers::image::noise::kuwahara::KuwaharaCommandHandler;
use crate::handlers::image::noise::mean::MeanCommandHandler;
use crate::handlers::image::noise::median::MedianCommandHandler;
use crate::handlers::image::noise::sharpen::SharpenCommandHandler;

pub struct ImageCommandHandler;

impl CommandHandler for ImageCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        match args.subcommand() {
            Some((name, args)) => {
                let mut handlers: HashMap<&str, Rc<dyn Fn(ArgMatches) -> CliPicturifyResult<()>>> = HashMap::new();

                // common
                handlers.insert(NoneCommand::name(), Rc::new(NoneCommandHandler::handle));
                
                // color
                handlers.insert(SepiaCommand::name(), Rc::new(SepiaCommandHandler::handle));
                handlers.insert(NegativeCommand::name(), Rc::new(NegativeCommandHandler::handle));
                
                // noise
                handlers.insert(KuwaharaCommand::name(), Rc::new(KuwaharaCommandHandler::handle));
                handlers.insert(MedianCommand::name(), Rc::new(MedianCommandHandler::handle));
                handlers.insert(MeanCommand::name(), Rc::new(MeanCommandHandler::handle));
                handlers.insert(SharpenCommand::name(), Rc::new(SharpenCommandHandler::handle));
                
                // edge
                handlers.insert(SobelCommand::name(), Rc::new(SobelCommandHandler::handle));
                handlers.insert(SobelRgbCommand::name(), Rc::new(SobelRgbCommandHandler::handle));

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

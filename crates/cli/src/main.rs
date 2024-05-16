use crate::commands::common::picturify::PicturifyCommand;
use crate::handlers::common::image::ImageCommandHandler;
use log::{error, info, LevelFilter};
use simplelog::*;

mod commands;
mod handlers;
mod metadata;
mod progress;

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let matches = PicturifyCommand::get().get_matches();

    match matches.subcommand() {
        Some(("image", arg)) => {
            ImageCommandHandler::handle(arg.clone());
        }
        Some(("movie", _)) => {
            info!("Movie command not implemented yet")
        }
        _ => {
            error!("No command specified")
        }
    }
}

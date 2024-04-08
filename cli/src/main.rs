use crate::commands::common::picturify::PicturifyCommand;
use crate::handlers::common::image::ImageCommandHandler;
use log::LevelFilter;
use simplelog::*;

mod commands;
mod handlers;
mod metadata;

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let matches = PicturifyCommand::new().get_matches();

    match matches.subcommand() {
        Some(("image", arg)) => {
            ImageCommandHandler::handle(arg.clone());
        }
        Some(("movie", _)) => {
            println!("Movie command");
        }
        _ => {
            println!("No command specified");
        }
    }
}

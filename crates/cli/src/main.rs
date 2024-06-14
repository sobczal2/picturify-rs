use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::error::CliPicturifyError;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image::ImageCommandHandler;
use crate::handlers::common::movie::MovieCommandHandler;
#[allow(unused_imports)]
use log::{error, info, warn, LevelFilter};
use simplelog::*;
use std::time::Instant;
use rand::random;

mod commands;
mod error;
mod handlers;
mod progress;
mod metadata;

fn main() {
    
    #[cfg(target_os = "windows")]
    if random::<i32>() % 3 == 0 {
        println!("Oh you're using Windows? That's awkward, I'm gonna crash now.");
        std::process::exit(1);
    }
    
    #[cfg(debug_assertions)]
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    #[cfg(not(debug_assertions))]
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    
    let start = Instant::now();

    let matches = PicturifyCommand::get().get_matches();

    let result = match matches.subcommand() {
        Some(("image", args)) => ImageCommandHandler::handle(&ImageCommandHandler, args.clone()),
        Some(("movie", args)) => MovieCommandHandler::handle(&MovieCommandHandler, args.clone()),
        _ => {
            PicturifyCommand::get().print_help().unwrap();
            Err(CliPicturifyError::MissingCommand)
        }
    };

    if let Err(e) = result {
        error!("{}", e);

        std::process::exit(1);
    }

    let duration = start.elapsed();
    match duration.as_secs() {
        0 => info!(
            "Damn, that was fast! Execution time: {}ms",
            duration.as_millis()
        ),
        _ => info!("Execution time: {}s", duration.as_secs()),
    }
}
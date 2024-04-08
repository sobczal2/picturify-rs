use crate::handlers::color::sepia::SepiaCommandHandler;
use clap::ArgMatches;
use log::{info};

pub struct ImageCommandHandler;

impl ImageCommandHandler {
    pub fn handle(arg_matches: ArgMatches) {
        let now = std::time::Instant::now();
        match arg_matches.subcommand() {
            Some(("sepia", args)) => {
                SepiaCommandHandler::handle(args.clone());
            }
            _ => {
                println!("No subcommand provided");
            }
        }
        let elapsed_ms = now.elapsed().as_millis();
        info!("Image command handled in {elapsed_ms}ms");
    }
}

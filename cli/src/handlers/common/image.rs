use crate::handlers::color::sepia::SepiaCommandHandler;
use crate::handlers::common::none::NoneCommandHandler;
use clap::ArgMatches;
use log::info;

pub struct ImageCommandHandler;

impl ImageCommandHandler {
    pub fn handle(arg_matches: ArgMatches) {
        let now = std::time::Instant::now();
        match arg_matches.subcommand() {
            Some(("sepia", args)) => {
                SepiaCommandHandler::handle(args.clone());
            }
            Some(("none", args)) => {
                NoneCommandHandler::handle(args.clone());
            }
            _ => {
                panic!("Unknown subcommand");
            }
        }
        let elapsed_ms = now.elapsed().as_millis();
        info!(
            "Command {} took {}ms",
            arg_matches.subcommand_name().unwrap(),
            elapsed_ms
        );
    }
}

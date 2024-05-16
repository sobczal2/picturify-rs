use clap::ArgMatches;
use crate::error::CliPicturifyResult;

pub trait CommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()>;
}
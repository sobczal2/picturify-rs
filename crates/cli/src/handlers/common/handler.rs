use crate::error::CliPicturifyResult;
use clap::ArgMatches;

pub trait CommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()>;
}

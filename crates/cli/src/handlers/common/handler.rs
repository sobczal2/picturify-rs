use crate::error::CliPicturifyResult;
use clap::ArgMatches;

pub trait CommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()>;
}

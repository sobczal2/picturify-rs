use clap::ArgMatches;

use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::completions::{CompletionsShell, CompletionsShellArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::CommandHandler;

pub struct CompletionsCommandHandler;

impl CommandHandler for CompletionsCommandHandler {
    fn handle(
        &self,
        args: ArgMatches
    ) -> CliPicturifyResult<()> {
        let shell = args.get_one::<CompletionsShell>(CompletionsShellArg::id()).map_to_unknown_error()?;
        
        shell.generate();
        
        Ok(())
    }
}
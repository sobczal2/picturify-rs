use crate::commands::common::command::Command;
use crate::commands::common::image::ImageCommand;
use crate::commands::common::movie::MovieCommand;
use crate::metadata::get_metadata;

pub struct PicturifyCommand;

impl Command for PicturifyCommand {
    fn get() -> clap::Command {
        let picturify_metadata = get_metadata();

        clap::Command::new(Self::name())
            .version(picturify_metadata.cli_version.to_string())
            .about(format!(
                "Picturify CLI - a CLI tool for image processing-bench using the Picturify library\n\
                CLI Version: {}\n\
                Core Version: {}\n\
                Processing Version: {}",
                picturify_metadata.cli_version,
                picturify_metadata.core_version,
                picturify_metadata.processing_version
            ))
            .subcommands(vec![ImageCommand::get(), MovieCommand::get()])
    }

    fn name() -> &'static str {
        "picturify"
    }
}

use crate::metadata::get_metadata;
use clap::Command;
use crate::commands::common::common::image::ImageCommand;
use crate::commands::common::common::movie::MovieCommand;

pub struct PicturifyCommand;

impl PicturifyCommand {
    pub fn new() -> Command {
        let picturify_metadata = get_metadata();

        Command::new("picturify")
            .version(picturify_metadata.cli_version.to_string())
            .about(format!(
                "Picturify CLI - a CLI tool for image processing using the Picturify library\n\
                CLI Version: {}\n\
                Core Version: {}\n\
                Processing Version: {}",
                picturify_metadata.cli_version,
                picturify_metadata.core_version,
                picturify_metadata.processing_version
            ))
            .subcommands(vec![ImageCommand::new(), MovieCommand::new()])
    }
}

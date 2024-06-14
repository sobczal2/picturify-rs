use crate::commands::common::command::{Command};
use crate::commands::common::image::ImageCommand;
use crate::commands::common::movie::MovieCommand;
use crate::metadata;

pub struct PicturifyCommand;

impl Command for PicturifyCommand {
    fn get() -> clap::Command {
        clap::Command::new(metadata::NAME)
            .version(metadata::VERSION)
            .author(metadata::AUTHORS)
            .about(format!(
                "{} - a CLI tool for core processing using the Picturify library\n\n\
                \t{} v{}\n\
                \t{} v{}\n\
                \t{} v{}\n\
                \t{} v{}\n",
                metadata::NAME,
                metadata::NAME,
                metadata::VERSION,
                picturify_core::metadata::NAME,
                picturify_core::metadata::VERSION,
                picturify_processing::metadata::NAME,
                picturify_processing::metadata::VERSION,
                picturify_movie::metadata::NAME,
                picturify_movie::metadata::VERSION,
            ))
            .subcommands(vec![ImageCommand::get(), MovieCommand::get()])
    }
}

use crate::commands::common::args::common::{InputArg, OutputArg, PicturifyArg};
use crate::common::filter_group::Group;
use colored::Colorize;

pub trait Command {
    fn create() -> clap::Command;
}

pub trait CommandForImage {
    fn get() -> clap::Command;
    fn get_base() -> clap::Command {
        clap::Command::new(Self::name())
            .about(Self::about())
            .arg(InputArg::create(None))
            .arg(OutputArg::create(None))
    }
    fn name() -> &'static str;
    fn display_name() -> String {
        Self::name().replace('-', " ")
    }
    fn group() -> Group;
    fn about() -> String {
        format!(
            "{}\tRun {} filter on the image",
            Self::group(),
            Self::display_name().bold()
        )
    }
}

pub trait CommandForMovie {
    fn get() -> clap::Command;
    fn get_base() -> clap::Command {
        clap::Command::new(Self::name()).about(Self::about())
    }
    fn name() -> &'static str;
    fn group() -> Group;
    fn about() -> String {
        format!("{} Run {} filter on the movie", Self::group(), Self::name())
    }
}

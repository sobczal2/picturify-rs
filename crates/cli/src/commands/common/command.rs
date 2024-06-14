use colored::Colorize;
use crate::commands::common::arg::ArgType;
use crate::common::filter_group::Group;

pub trait Command {
    fn get() -> clap::Command;
}

pub trait CommandForImage {
    fn get() -> clap::Command;
    fn get_base() -> clap::Command {
        clap::Command::new(Self::name())
            .about(Self::about())
            .arg(ArgType::Input.to_arg())
            .arg(ArgType::Output.to_arg())
    }
    fn name() -> &'static str;
    fn display_name() -> String {
        Self::name().replace("-", " ")  
    }
    fn group() -> Group;
    fn about() -> String {
        format!("{}\tRun {} filter on the image", Self::group(), Self::display_name().bold())
    }
}

pub trait CommandForMovie {
    fn get() -> clap::Command;
    fn get_base() -> clap::Command {
        clap::Command::new(Self::name())
            .about(Self::about())
    }
    fn name() -> &'static str;
    fn group() -> Group;
    fn about() -> String {
        format!("{} Run {} filter on the movie", Self::group(), Self::name())
    }
}


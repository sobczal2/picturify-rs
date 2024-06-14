use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct MeanBlurCommand;

impl CommandForImage for MeanBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Radius.to_arg())
            .arg(ArgType::Fast.to_arg())
    }

    fn name() -> &'static str {
        "mean-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}

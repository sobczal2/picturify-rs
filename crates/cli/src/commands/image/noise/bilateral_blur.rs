use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct BilateralBlurCommand;

impl CommandForImage for BilateralBlurCommand {
    fn get() -> clap::Command {
        Self::get_base()
            .arg(ArgType::Radius.to_arg())
            .arg(ArgType::Fast.to_arg())
            .arg(ArgType::SigmaSpatial.to_arg())
            .arg(ArgType::SigmaIntensity.to_arg())
    }

    fn name() -> &'static str {
        "bilateral-blur"
    }

    fn group() -> Group {
        Group::Noise
    }
}

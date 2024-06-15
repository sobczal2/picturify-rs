use crate::commands::common::arg::ArgType;
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

pub struct SepiaCommand;

impl CommandForImage for SepiaCommand {
    fn get() -> clap::Command {
        #[cfg(feature = "gpu")]
        {
            Self::get_base()
                .arg(ArgType::Fast.to_arg())
                .arg(ArgType::Gpu.to_arg())
        }

        #[cfg(not(feature = "gpu"))]
        {
            Self::get_base().arg(ArgType::Fast.to_arg())
        }
    }

    fn name() -> &'static str {
        "sepia"
    }

    fn group() -> Group {
        Group::Color
    }
}

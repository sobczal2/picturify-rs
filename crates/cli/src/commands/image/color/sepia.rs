#[cfg(feature = "gpu")]
use crate::commands::common::args::common::GpuArg;
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::common::command::CommandForImage;
use crate::common::filter_group::Group;

struct SepiaDefaultArgs {
    fast: &'static str,
    #[cfg(feature = "gpu")]
    gpu: &'static str,
}

const DEFAULT_ARGS: SepiaDefaultArgs = SepiaDefaultArgs {
    fast: "false",
    #[cfg(feature = "gpu")]
    gpu: "false",
};

pub struct SepiaCommand;

impl CommandForImage for SepiaCommand {
    fn get() -> clap::Command {
        #[cfg(feature = "gpu")]
        {
            Self::get_base()
                .arg(FastArg::create(DEFAULT_ARGS.fast))
                .arg(GpuArg::create(DEFAULT_ARGS.gpu))
        }

        #[cfg(not(feature = "gpu"))]
        {
            Self::get_base().arg(FastArg::create(DEFAULT_ARGS.fast))
        }
    }

    fn name() -> &'static str {
        "sepia"
    }

    fn group() -> Group {
        Group::Color
    }
}

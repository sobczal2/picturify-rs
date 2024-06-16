use clap::builder::{IntoResettable, OsStr};
use clap::{value_parser, Arg, ArgAction};
use std::path::PathBuf;

pub trait PicturifyArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg;
    fn id() -> &'static str;
}

pub struct InputArg;

impl PicturifyArg for InputArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('i')
            .long("input")
            .help("Input path")
            .required(true)
            .default_value(default_value)
            .value_parser(value_parser!(PathBuf))
    }

    fn id() -> &'static str {
        "input"
    }
}

pub struct OutputArg;

impl PicturifyArg for OutputArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('o')
            .long("output")
            .help("Output path")
            .required(true)
            .default_value(default_value)
            .value_parser(value_parser!(PathBuf))
    }

    fn id() -> &'static str {
        "output"
    }
}

pub struct FastArg;

impl PicturifyArg for FastArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('f')
            .long("fast")
            .help("Fast mode")
            .action(ArgAction::SetTrue)
            .default_value(default_value)
            .value_parser(value_parser!(bool))
    }

    fn id() -> &'static str {
        "fast"
    }
}

pub struct GpuArg;

impl PicturifyArg for GpuArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('g')
            .long("gpu")
            .help("Use GPU")
            .default_value(default_value)
            .value_parser(value_parser!(bool))
    }

    fn id() -> &'static str {
        "gpu"
    }
}

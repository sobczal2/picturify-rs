use std::path::PathBuf;
use clap::{Arg, value_parser};
use clap::builder::{IntoResettable, OsStr};

pub trait PicturifyArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg;
    fn id() -> &'static str;
}

pub struct InputArg;

impl PicturifyArg for InputArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
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
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
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
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .short('f')
            .long("fast")
            .help("Fast mode")
            .default_value(default_value)
            .value_parser(value_parser!(bool))
    }

    fn id() -> &'static str {
        "fast"
    }
}

pub struct GpuArg;

impl PicturifyArg for GpuArg {
    fn new(default_value: impl IntoResettable<OsStr>) -> Arg {
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

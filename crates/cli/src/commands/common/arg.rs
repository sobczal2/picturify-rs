use clap::{value_parser, Arg, ArgAction, Command};
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;
use crate::commands::parsers::angle::AngleValueParser;

pub enum ArgType {
    Input,
    Output,
    Radius,
    Fast,
    GrayscaleStrategy,
    Sigma,
    SigmaSpatial,
    SigmaIntensity,
    Factor,
    Angle,
}

impl ArgType {
    pub fn to_arg(&self) -> Arg {
        match self {
            ArgType::Input => Arg::new(self.to_id())
                .short('i')
                .long("input")
                .help("Input image path")
                .required(true)
                .value_parser(value_parser!(String)),
            ArgType::Output => Arg::new(self.to_id())
                .short('o')
                .long("output")
                .help("Output image path")
                .required(true)
                .value_parser(value_parser!(String)),
            ArgType::Radius => Arg::new(self.to_id())
                .short('r')
                .long("radius")
                .help("Radius of the filter")
                .default_value("3")
                .value_parser(value_parser!(usize)),
            ArgType::Fast => Arg::new(self.to_id())
                .short('f')
                .long("fast")
                .action(ArgAction::SetTrue)
                .help("Use faster, but less accurate approximation of the algorithm")
                .default_value("false")
                .value_parser(value_parser!(bool)),
            ArgType::GrayscaleStrategy => Arg::new(self.to_id())
                .short('s')
                .long("strategy")
                .help("Grayscale strategy")
                .default_value("luminosity")
                .value_parser(value_parser!(GrayscaleStrategy)),
            ArgType::Sigma => Arg::new(self.to_id())
                .short('s')
                .long("sigma")
                .help("Sigma value")
                .default_value("1.0")
                .value_parser(value_parser!(f32)),
            ArgType::SigmaSpatial => Arg::new(self.to_id())
                .long("sigma-spatial")
                .help("Sigma spatial value")
                .default_value("1.0")
                .value_parser(value_parser!(f32)),
            ArgType::SigmaIntensity => Arg::new(self.to_id())
                .long("sigma-intensity")
                .help("Sigma intensity value")
                .default_value("1.0")
                .value_parser(value_parser!(f32)),
            ArgType::Factor => Arg::new(self.to_id())
                .short('f')
                .long("factor")
                .help("Brightness factor")
                .default_value("1.0")
                .value_parser(value_parser!(f32)),
            ArgType::Angle => Arg::new(self.to_id())
                .short('a')
                .long("angle")
                .help("Angle value in radians or degrees (e.g. 90deg or 1.57rad)")
                .default_value("0deg")
                .value_parser(AngleValueParser::new()),
        }
    }

    pub fn to_id(&self) -> &'static str {
        match self {
            ArgType::Input => "input",
            ArgType::Output => "output",
            ArgType::Radius => "radius",
            ArgType::Fast => "fast",
            ArgType::GrayscaleStrategy => "grayscale-strategy",
            ArgType::Sigma => "sigma",
            ArgType::SigmaSpatial => "sigma-spatial",
            ArgType::SigmaIntensity => "sigma-intensity",
            ArgType::Factor => "factor",
            ArgType::Angle => "angle",
        }
    }
}

pub fn add_input_output_args(command: Command) -> Command {
    command
        .arg(ArgType::Input.to_arg())
        .arg(ArgType::Output.to_arg())
}

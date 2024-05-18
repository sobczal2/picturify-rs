use clap::{Arg, ArgAction, Command, value_parser};
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;

pub enum ArgType {
    Input,
    Output,
    Radius,
    Fast,
    GrayscaleStrategy,
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
        }
    }
    
    pub fn to_id(&self) -> &'static str {
        match self {
            ArgType::Input => "input",
            ArgType::Output => "output",
            ArgType::Radius => "radius",
            ArgType::Fast => "fast",
            ArgType::GrayscaleStrategy => "grayscale-strategy",
        }
    }
}

pub fn add_input_output_args(command: Command) -> Command {
    command
        .arg(ArgType::Input.to_arg())
        .arg(ArgType::Output.to_arg())
}
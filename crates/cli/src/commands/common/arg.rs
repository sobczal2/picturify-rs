use clap::{Arg, Command, value_parser};

pub enum ArgType {
    Input,
    Output,
    Radius,
    UseFastApproximation,
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
            ArgType::UseFastApproximation => Arg::new(self.to_id())
                .short('f')
                .long("use-fast-approximation")
                .help("Use fast approximation for filter")
                .default_value("false")
                .value_parser(value_parser!(bool)),
        }
    }
    
    pub fn to_id(&self) -> &'static str {
        match self {
            ArgType::Input => "input",
            ArgType::Output => "output",
            ArgType::Radius => "radius",
            ArgType::UseFastApproximation => "use-fast-approximation",
        }
    }
}

pub fn add_input_output_args(command: Command) -> Command {
    command
        .arg(ArgType::Input.to_arg())
        .arg(ArgType::Output.to_arg())
}
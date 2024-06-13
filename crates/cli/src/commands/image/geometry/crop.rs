use crate::commands::common::arg::{add_input_output_args, ArgType};
use crate::commands::common::command::Command;

pub struct CropCommand;

impl Command for CropCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(ArgType::Border.to_arg())
            .about("Crop the core");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "crop"
    }
}

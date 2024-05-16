use crate::commands::common::arg::add_input_output_args;
use crate::commands::common::command::Command;

pub struct KuwaharaCommand;

impl Command for KuwaharaCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .about("Run kuwahara processing pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "kuwahara"
    }
}

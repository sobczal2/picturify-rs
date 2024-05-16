use crate::commands::common::arg::add_input_output_args;
use crate::commands::common::command::Command;

pub struct KuwaharaCommand;

impl Command for KuwaharaCommand {
    fn get() -> clap::Command {
        let cmd = clap::Command::new(Self::name())
            .arg(
                crate::commands::common::arg::ArgType::Radius.to_arg(),
            )
            .about("Run kuwahara processing pipeline on the image");
        add_input_output_args(cmd)
    }

    fn name() -> &'static str {
        "kuwahara"
    }
}

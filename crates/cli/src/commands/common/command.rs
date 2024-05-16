pub trait Command {
    fn get() -> clap::Command;
    fn name() -> &'static str;
}
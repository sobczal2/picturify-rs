use std::env::var;
use std::io;
use clap::{Arg, Error};
use clap::builder::{IntoResettable, OsStr, TypedValueParser};
use clap::error::ErrorKind;
use clap_complete::{generate};
use clap_complete::shells::{Bash, Fish, PowerShell, Zsh};
use crate::commands::common::args::common::PicturifyArg;
use crate::commands::common::command::Command;
use crate::commands::common::picturify::PicturifyCommand;
use crate::metadata;

#[derive(Debug, Clone, Copy)]
pub enum CompletionsShell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
}

impl CompletionsShell {
    pub fn generate(&self) {
        match self {
            Self::Bash => generate(Bash, &mut PicturifyCommand::create(), metadata::NAME, &mut io::stdout()),
            Self::Fish => generate(Fish, &mut PicturifyCommand::create(), metadata::NAME, &mut io::stdout()),
            Self::Zsh => generate(Zsh, &mut PicturifyCommand::create(), metadata::NAME, &mut io::stdout()),
            Self::PowerShell => generate(PowerShell, &mut PicturifyCommand::create(), metadata::NAME, &mut io::stdout()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompletionsShellValueParser;

impl TypedValueParser for CompletionsShellValueParser {
    type Value = CompletionsShell;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        match value.to_str() {
            Some("bash") => Ok(CompletionsShell::Bash),
            Some("fish") => Ok(CompletionsShell::Fish),
            Some("zsh") => Ok(CompletionsShell::Zsh),
            Some("powershell") => Ok(CompletionsShell::PowerShell),
            _ => Err(
                Error::raw(
                    ErrorKind::InvalidValue,
                    "Invalid angle, expected format: <value>[rad|deg]\n",
                )
            )
        }
    }
}

pub struct CompletionsShellArg;

impl PicturifyArg for CompletionsShellArg {
    fn create(default_value: impl IntoResettable<OsStr>) -> Arg {
        Arg::new(Self::id())
            .long("shell")
            .help("The shell to generate completions for")
            .default_value(default_value)
            .value_parser(CompletionsShellValueParser)
    }

    fn id() -> &'static str {
        "shell"
    }
}

pub struct CompletionsCommand;

impl Command for CompletionsCommand {
    fn create() -> clap::Command {
        let default_shell = var("SHELL")
            .map(|shell| {
                match shell.as_str() {
                    "/bin/bash" => "bash",
                    "/bin/fish" => "fish",
                    "/bin/zsh" => "zsh",
                    _ => "bash",
                }
            })
            .unwrap_or("bash");

        clap::Command::new("completions")
            .hide(true)
            .about("Generate shell completions")
            .arg(CompletionsShellArg::create(default_shell))
    }
}
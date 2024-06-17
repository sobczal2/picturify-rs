use std::ffi::OsStr;
use clap::{Arg, Command, Error};
use clap::builder::TypedValueParser;
use clap::error::ErrorKind;

#[derive(Debug, Clone)]
pub enum CpuCount {
    Auto,
    Count(usize),
}

#[derive(Debug, Clone)]
pub struct CpuCountValueParser;

impl TypedValueParser for CpuCountValueParser {
    type Value = CpuCount;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        match value.to_str() {
            Some("auto") => Ok(CpuCount::Auto),
            Some(value) => value
                .parse()
                .map(CpuCount::Count)
                .map_err(|_| Error::raw(
                    ErrorKind::InvalidValue,
                    "Invalid CPU count, expected a positive integer or 'auto'\n",
                )),
            None => Err(Error::raw(
                ErrorKind::InvalidValue,
                "Invalid CPU count, expected a positive integer or 'auto'\n",
            )),
        }
    }
}

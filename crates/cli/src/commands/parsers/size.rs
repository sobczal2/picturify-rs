use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use clap::{Arg, Command, Error};
use picturify_core::geometry::size::Size;
use std::ffi::OsStr;

#[derive(Debug, Copy, Clone)]
pub struct SizeValueParser;

impl SizeValueParser {
    pub fn new() -> Self {
        Self
    }
}

impl TypedValueParser for SizeValueParser {
    type Value = Size;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        let regex = regex::Regex::new(r"(\d+)x(\d+)").unwrap();
        let value = value.to_str().unwrap();
        let captures = regex.captures(value).ok_or_else(|| {
            Error::raw(
                ErrorKind::InvalidValue,
                "Invalid size, expected format: <width>x<height>\n",
            )
        })?;

        let width = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        Ok(Size::new(width, height))
    }
}

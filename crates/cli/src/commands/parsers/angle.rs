use std::ffi::OsStr;

use clap::{Arg, Command, Error};
use clap::builder::TypedValueParser;
use clap::error::ErrorKind;

use picturify_core::geometry::angle::Angle;

#[derive(Debug, Copy, Clone)]
pub struct AngleValueParser;

impl AngleValueParser {
    pub fn new() -> Self {
        Self
    }
}

impl TypedValueParser for AngleValueParser {
    type Value = Angle;

    #[allow(unused_variables)]
    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value.to_str().unwrap();
        value.parse::<Angle>().map_err(|_| {
            Error::raw(
                ErrorKind::InvalidValue,
                "Invalid angle, expected format: <value>[rad|deg]\n",
            )
        })
    }
}

use std::ffi::OsStr;

use clap::{Arg, Command, Error};
use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use regex::Regex;

use picturify_processing::processors::geometry::crop::CropBorder;

#[derive(Debug, Copy, Clone)]
pub struct CropBorderValueParser;

impl CropBorderValueParser {
    pub fn new() -> Self {
        Self
    }
}

impl TypedValueParser for CropBorderValueParser {
    type Value = CropBorder;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        let regex = Regex::new(r"(\d+)x(\d+)\+(\d+)\+(\d+)").unwrap();
        let value = value.to_str().unwrap();
        let captures = regex.captures(value).ok_or_else(|| {
            Error::raw(
                ErrorKind::InvalidValue,
                "Invalid crop border, expected format: <width>x<height>+<x>+<y>\n",
            )
        })?;

        let width = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let x_offset = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let y_offset = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();

        Ok(CropBorder::new(width, height, x_offset, y_offset))
    }
}

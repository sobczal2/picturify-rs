use clap::builder::TypedValueParser;
use clap::error::ErrorKind;
use clap::{Arg, Command, Error};
use picturify_processing::processors::geometry::scale::ScaleStrategy;
use std::ffi::OsStr;

#[derive(Debug, Copy, Clone)]
pub struct ScaleStrategyValueParser;

impl ScaleStrategyValueParser {
    pub fn new() -> Self {
        Self
    }
}

impl TypedValueParser for ScaleStrategyValueParser {
    type Value = ScaleStrategy;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        match value.to_str().unwrap() {
            "nearest-neighbor" => Ok(ScaleStrategy::NearestNeighbor),
            "nn" => Ok(ScaleStrategy::NearestNeighbor),
            "bilinear" => Ok(ScaleStrategy::Bilinear),
            "bl" => Ok(ScaleStrategy::Bilinear),
            _ => Err(Error::raw(
                ErrorKind::InvalidValue,
                "Invalid scale strategy, expected one of: nearest_neighbor(nn), bilinear(bl)\n",
            )),
        }
    }
}

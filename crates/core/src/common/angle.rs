use crate::error::PicturifyError;
use std::f32::consts::PI;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

impl Angle {
    pub fn to_radians(&self) -> f32 {
        match *self {
            Angle::Radians(radians) => radians,
            Angle::Degrees(degrees) => degrees.to_radians() % (2.0 * PI),
        }
    }

    pub fn to_degrees(&self) -> f32 {
        match *self {
            Angle::Radians(radians) => radians.to_degrees() % 360.0,
            Angle::Degrees(degrees) => degrees,
        }
    }
}

impl FromStr for Angle {
    type Err = PicturifyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.ends_with("rad") {
            let radians = s[..s.len() - 3]
                .parse::<f32>()
                .map_err(|_| PicturifyError::ParseError("Invalid angle".to_string()))?;
            Ok(Angle::Radians(radians))
        } else if s.ends_with("deg") {
            let degrees = s[..s.len() - 3]
                .parse::<f32>()
                .map_err(|_| PicturifyError::ParseError("Invalid angle".to_string()))?;
            Ok(Angle::Degrees(degrees))
        } else {
            let radians = s
                .parse::<f32>()
                .map_err(|_| PicturifyError::ParseError("Invalid angle".to_string()))?;
            Ok(Angle::Radians(radians))
        }
    }
}

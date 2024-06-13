use crate::error::PicturifyError;
use std::f32::consts::PI;
use std::ops::Neg;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

impl Angle {
    pub fn from_degrees(degrees: f32) -> Self {
        Angle::Degrees(degrees)
    }

    pub fn from_radians(radians: f32) -> Self {
        Angle::Radians(radians)
    }

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

    pub fn to_sin_cos(&self) -> (f32, f32) {
        let radians = self.to_radians();
        (radians.sin(), radians.cos())
    }
}

impl FromStr for Angle {
    type Err = PicturifyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 10 {
            return Err(PicturifyError::ParseError("Invalid angle".to_string()));
        }

        let s = s.trim();
        if let Some(radians_str) = s.strip_suffix("rad") {
            let radians = radians_str
                .parse::<f32>()
                .map_err(|_| PicturifyError::ParseError("Invalid angle".to_string()))?;
            Ok(Angle::Radians(radians))
        } else if let Some(degrees_str) = s.strip_suffix("deg") {
            let degrees = degrees_str
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

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        match self {
            Angle::Radians(radians) => Angle::Radians(-radians),
            Angle::Degrees(degrees) => Angle::Degrees(-degrees),
        }
    }
}

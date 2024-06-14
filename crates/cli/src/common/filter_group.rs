use colored::Colorize;
use std::fmt::Display;

pub enum Group {
    Color,
    Common,
    Edge,
    Geometry,
    Noise,
}

impl Group {
    fn names() -> Vec<&'static str> {
        vec!["COLOR", "COMMON", "EDGE", "GEOMETRY", "NOISE"]
    }

    fn max_len() -> usize {
        Group::names().iter().map(|name| name.len()).max().unwrap()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_len = Group::max_len() + 2;

        let name = match self {
            Group::Color => "COLOR",
            Group::Common => "COMMON",
            Group::Edge => "EDGE",
            Group::Geometry => "GEOMETRY",
            Group::Noise => "NOISE",
        };

        let name = format!("[{}]", name);

        let result = format!("{:<width$}", name, width = max_len);

        match self {
            Group::Color => write!(f, "{}", result.red()),
            Group::Common => write!(f, "{}", result.green()),
            Group::Edge => write!(f, "{}", result.blue()),
            Group::Geometry => write!(f, "{}", result.yellow()),
            Group::Noise => write!(f, "{}", result.magenta()),
        }
    }
}

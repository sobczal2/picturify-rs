use std::fmt::Display;

pub struct PicturifyLogger;

impl PicturifyLogger {
    pub fn log_error(msg: impl Display) {
        let formatted = format!("{}", msg);
        let lines = formatted.lines();
        for line in lines {
            log::error!("{}", line);
        }
    }

    pub fn log_warn(msg: impl Display) {
        let formatted = format!("{}", msg);
        let lines = formatted.lines();
        for line in lines {
            log::warn!("{}", line);
        }
    }

    pub fn log_info(msg: impl Display) {
        let formatted = format!("{}", msg);
        let lines = formatted.lines();
        for line in lines {
            log::info!("{}", line);
        }
    }

    pub fn log_debug(msg: impl Display) {
        let formatted = format!("{}", msg);
        let lines = formatted.lines();
        for line in lines {
            log::debug!("{}", line);
        }
    }

    pub fn log_trace(msg: impl Display) {
        let formatted = format!("{}", msg);
        let lines = formatted.lines();
        for line in lines {
            log::trace!("{}", line);
        }
    }
}

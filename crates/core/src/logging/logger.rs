#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        let formatted = format!("{}", $msg);
        let lines = formatted.lines();
        for line in lines {
            log::error!("{}", line);
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        let formatted = format!("{}", $msg);
        let lines = formatted.lines();
        for line in lines {
            log::warn!("{}", line);
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        let formatted = format!("{}", $msg);
        let lines = formatted.lines();
        for line in lines {
            log::info!("{}", line);
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        let formatted = format!("{}", $msg);
        let lines = formatted.lines();
        for line in lines {
            log::debug!("{}", line);
        }
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        let formatted = format!("{}", $msg);
        let lines = formatted.lines();
        for line in lines {
            log::trace!("{}", line);
        }
    };
}

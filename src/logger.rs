use log::{set_max_level, LevelFilter, Metadata, Record};

#[derive(Debug)]
pub struct Logger {}

static INSTANCE: Logger = Logger {};

impl Logger {
    pub fn init(level: LevelFilter) -> &'static Logger {
        set_max_level(level);
        Self::global()
    }

    pub fn global() -> &'static Logger {
        &INSTANCE
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
    }

    fn flush(&self) {}
}

const INDENTS: &str = "                        "; // 24 spaces
const INDENTS_WIDTH: usize = 2;

pub fn indent(i: usize) -> &'static str {
    &INDENTS[..std::cmp::min(i * INDENTS_WIDTH, INDENTS.len())]
}

pub fn indent_display(f: &std::fmt::Formatter<'_>) -> &'static str {
    if let Some(std::fmt::Alignment::Right) = f.align() {
        indent(f.width().unwrap_or(1))
    } else {
        ""
    }
}

#[macro_export]
macro_rules! info {
    (ind: $ind:expr, $($arg:tt)+) => {
        log::info!("{}{}", $crate::logger::indent($ind), format!($($arg)+));
    };
    ($($arg:tt)+) => {
        log::info!($($arg)+);
    }
}

#[macro_export]
macro_rules! error {
    (ind: $ind:expr, $($arg:tt)+) => {
        log::error!("{}{}", $crate::logger::indent($ind), colored::Colorize::red(&*format!($($arg)+)));
    };
    ($($arg:tt)+) => {
        log::error!($($arg)+);
    }
}

#[macro_export]
macro_rules! debug {
    (ind: $ind:expr, $($arg:tt)+) => {
        log::debug!("{}{}", $crate::logger::indent($ind), format!($($arg)+));
    };
    ($($arg:tt)+) => {
        log::debug!($($arg)+);
    }
}

#[macro_export]
macro_rules! trace {
    (ind: $ind:expr, $($arg:tt)+) => {
        log::trace!("{}{}", $crate::logger::indent($ind), format!($($arg)+));
    };
    ($($arg:tt)+) => {
        log::trace!($($arg)+);
    }
}

#[macro_export]
macro_rules! warn {
    (ind: $ind:expr, $($arg:tt)+) => {
        log::warn!("{}{}", $crate::logger::indent($ind), colored::Colorize::yellow(&*format!($($arg)+)));
    };
    ($($arg:tt)+) => {
        log::warn!($($arg)+);
    }
}

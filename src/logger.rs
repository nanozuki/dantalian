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
    &INDENTS[..std::cmp::min(i*INDENTS_WIDTH, (&INDENTS).len())]
}

pub fn indent_display(f: &std::fmt::Formatter<'_>) -> &'static str {
    if let Some(align) = f.align() {
        if let std::fmt::Alignment::Right = align {
            return indent(f.width().unwrap_or(1))
        }
    }
    return "";
}

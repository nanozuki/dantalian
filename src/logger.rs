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

const INDENTS: &str = "\t\t\t\t\t\t";
const MAX_INDENTS: u8 = 6;

pub fn indent(i: u8) -> String {
    if i > MAX_INDENTS {
        String::from(INDENTS)
    } else {
        String::from(&INDENTS[..i as usize])
    }
}

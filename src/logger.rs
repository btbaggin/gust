use std::fs::{OpenOptions, File};
use std::sync::Mutex;
use std::fmt::Debug;

pub use log::{error, warn, info, trace, debug};
use log::{Log, Level, LevelFilter, Metadata, Record};

struct Logger;
impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            use std::io::Write;
            let mut file = FILE.lock().unwrap();

            let time_string = chrono::Local::now().format("%x %X");
            let level = record.level();
            let trace = match level {
                Level::Error => format!("{:?}", backtrace::Backtrace::new()),
                _ => String::from(""),
            };
            let message = format!("{}: {} [{}]: {} {:?}\n", record.metadata().target(), level, time_string, record.args(), trace);

            file.write_all(message.as_bytes()).unwrap();
        }
    }

    fn flush(&self) { }
}

pub fn set_log_level(level: &str) {
    use core::str::FromStr;
    let level = LevelFilter::from_str(level).log_and_panic();
    log::set_max_level(level)
}

static LOGGER: Logger = Logger;
lazy_static::lazy_static! {
    pub static ref FILE: Mutex<File> = Mutex::new(OpenOptions::new().create(true).write(true).open("./log.txt").unwrap());
}

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}

pub trait PanicLogEntry<T> {
    fn log_message_and_panic(self, message: &str) -> T;
    fn log_and_panic(self) -> T;
}
pub trait LogEntry<T: Default> {
    fn log(self, message: &str) -> T;
}
impl<T, E: Debug> PanicLogEntry<T> for std::result::Result<T, E> {
    /// Logs the type with an additional message if it is `Err` then panics  
    fn log_message_and_panic(self, message: &str) -> T {
        match self {
            Err(e) => {
                log::error!("{:?} - {}", e, message);
                panic!("encountered unexpected error");
            }
            Ok(r) => r,
        }
    }

    /// Logs the type if it is `Err` then panics
    fn log_and_panic(self) -> T {
        match self {
            Err(e) => {
                log::error!("{:?}", e);
                panic!("encountered unexpected error");
            }
            Ok(r) => r,
        }
    }
}

impl <T: Default, E: Debug> LogEntry<T> for std::result::Result<T, E> {
    fn log(self, message: &str) -> T {
        match self {
            Err(e) => {
                log::warn!("{:?} - {}", e, message);
                std::default::Default::default()
            }
            Ok(r) => r,
        }
    }
}

impl<T> PanicLogEntry<T> for Option<T> {
    /// Logs the type with an additional message if it is `None` then panics
    fn log_message_and_panic(self, message: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                log::error!("None - {}", message);
                panic!("encountered unexpected error");
            }
        }
    }
     

    /// Logs the type if it is `None` then panics
    fn log_and_panic(self) -> T {
        match self {
            Some(t) => t,
            None => {
                log::error!("None", );
                panic!("encountered unexpected error");
            }
        }
    }
}
use std::{collections::VecDeque, sync::{atomic::{AtomicBool, Ordering}, Mutex}};

const LOG_CAPACITY: usize = 500;

pub struct Logger {
    pub level: Mutex<log::LevelFilter>,
    pub lines: Mutex<VecDeque<String>>,
    updated: AtomicBool,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: Mutex::new(log::LevelFilter::Warn),
            lines: Mutex::new(VecDeque::with_capacity(LOG_CAPACITY)),
            updated: AtomicBool::new(false),
        }
    }

    pub fn filter(&self, filter: log::LevelFilter) {
        *self.level.lock().unwrap() = filter;
    }

    pub fn updated(&self) -> bool {
        self.updated.load(Ordering::SeqCst)
    }

    pub fn set_updated(&self, state: bool) {
        self.updated.store(state, Ordering::SeqCst);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= *self.level.lock().unwrap()
    }

    fn log(&self, record: &log::Record) {
        if let Some(module) = record.module_path() {
            if module.to_owned().contains("tes_parse") || module.to_owned().contains("open_creation") {
                let mut lines = self.lines.lock().unwrap();

                if lines.len() > LOG_CAPACITY {
                    lines.pop_front();
                }

                lines.push_back(format!(
                    "[{}][{}] {}",
                    record.level(),
                    record.module_path().unwrap_or("unknown module"),
                    record.args(),
                ));

                self.updated.store(true, Ordering::SeqCst);
            }
        }
    }

    fn flush(&self) {}
}

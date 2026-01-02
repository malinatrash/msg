use std::sync::Arc;

use crate::config::logger::LoggerConfig;

pub struct Logger;

impl Logger {
    pub fn new(_config: &LoggerConfig) -> Arc<Self> {
        Arc::new(Logger)
    }
}

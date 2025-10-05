use std::fmt;
use std::process::exit;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::Serialize;

// 定义日志等级
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => {
                println!("无效的日志级别: {}", s);
                Ok(LogLevel::Info)
            },
        }
    }
}

// 日志配置
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            level: LogLevel::Info,
        }
    }
}

// 日志记录器
pub struct Logger {
    config: Mutex<LoggerConfig>, // 使用Mutex实现内部可变性
}

impl Logger {
    // 创建新的日志记录器
    pub fn new(config: Option<LoggerConfig>) -> Self {
        Logger {
            config: Mutex::new(config.unwrap_or(LoggerConfig::default())),
        }
    }

    pub fn default() -> Self {
        Logger::new(Some(LoggerConfig::default()))
    }

    // 设置日志等级
    pub fn set_level(&self, level: LogLevel) {
        let mut config = self.config.lock().unwrap();
        config.level = level;
    }

    // 带格式化的日志方法，支持模板和任意变量
    pub fn log_fmt(&self, level: LogLevel, args: fmt::Arguments<'_>) {
        let config = self.config.lock().unwrap();
        if level < config.level {
            return;
        }

        let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] {}", time, level, args);
        if level == LogLevel::Error {
            drop(config);
            exit(-1);
        }
    }

    // 带格式化的便捷方法
    pub fn trace_fmt(&self, args: fmt::Arguments<'_>) {
        self.log_fmt(LogLevel::Trace, args);
    }

    pub fn debug_fmt(&self, args: fmt::Arguments<'_>) {
        self.log_fmt(LogLevel::Debug, args);
    }

    pub fn info_fmt(&self, args: fmt::Arguments<'_>) {
        self.log_fmt(LogLevel::Info, args);
    }

    pub fn warn_fmt(&self, args: fmt::Arguments<'_>) {
        self.log_fmt(LogLevel::Warn, args);
    }

    pub fn error_fmt(&self, args: fmt::Arguments<'_>) {
        self.log_fmt(LogLevel::Error, args);
    }
}

// 全局日志记录器
lazy_static! {
    pub static ref GLOBAL_LOGGER: Logger = Logger::default();
}

pub fn set_global_level(level: LogLevel) {
    GLOBAL_LOGGER.set_level(level);
}

// 宏定义修改：使用crate路径而非$crate，更适合模块内使用
#[macro_export]
macro_rules! trace_fmt {
    ($($arg:tt)*) => {
        crate::utils::logger::GLOBAL_LOGGER.trace_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug_fmt {
    ($($arg:tt)*) => {
        crate::utils::logger::GLOBAL_LOGGER.debug_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! info_fmt {
    ($($arg:tt)*) => {
        crate::utils::logger::GLOBAL_LOGGER.info_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn_fmt {
    ($($arg:tt)*) => {
        crate::utils::logger::GLOBAL_LOGGER.warn_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error_fmt {
    ($($arg:tt)*) => {
        crate::utils::logger::GLOBAL_LOGGER.error_fmt(format_args!($($arg)*));
    };
}
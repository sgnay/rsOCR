use crate::models::{Result, RsOcrError};
use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// 自定义日志记录器
pub struct RsOcrLogger {
    log_file: Mutex<File>,
}

impl RsOcrLogger {
    /// 创建新的日志记录器
    pub fn new() -> Result<Self> {
        let log_path = Self::log_path();

        // 确保日志目录存在
        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent).map_err(RsOcrError::Io)?;
        }

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(RsOcrError::Io)?;

        Ok(Self {
            log_file: Mutex::new(log_file),
        })
    }

    /// 获取日志文件路径
    pub fn log_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".rsOCR/rsOCR.log")
    }

    /// 初始化日志系统
    pub fn init() -> Result<()> {
        let logger = Self::new()?;

        log::set_boxed_logger(Box::new(logger)).map_err(|e| RsOcrError::Generic(e.to_string()))?;
        log::set_max_level(log::LevelFilter::Info);

        Ok(())
    }
}

impl log::Log for RsOcrLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");
            let level = record.level();
            let message = record.args();

            let log_line = format!("[{}] [{}] {}\n", timestamp, level, message);

            if let Ok(mut file) = self.log_file.lock() {
                let _ = file.write_all(log_line.as_bytes());
            }

            // 同时输出到标准输出，保持原有行为
            println!("{}", message);
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.log_file.lock() {
            let _ = file.flush();
        }
    }
}

/// 记录信息级别日志
///
/// 这个函数记录信息级别的日志消息，同时输出到日志文件和标准输出。
///
/// # 参数
/// * `message` - 要记录的日志消息
///
/// # 示例
///
/// ```
/// use rsocr::info;
///
/// info("应用程序已启动");
/// ```
pub fn info(message: &str) {
    log::info!("{}", message);
}

/// 记录错误级别日志
///
/// 这个函数记录错误级别的日志消息，同时输出到日志文件和标准输出。
///
/// # 参数
/// * `message` - 要记录的日志消息
///
/// # 示例
///
/// ```
/// use rsocr::error;
///
/// error("文件读取失败");
/// ```
pub fn error(message: &str) {
    log::error!("{}", message);
}

/// 记录调试级别日志
///
/// 这个函数记录调试级别的日志消息，同时输出到日志文件和标准输出。
/// 注意：默认日志级别为Info，调试日志可能被过滤。
///
/// # 参数
/// * `message` - 要记录的日志消息
///
/// # 示例
///
/// ```
/// use rsocr::debug;
///
/// debug("调试信息: 变量值 = 42");
/// ```
pub fn debug(message: &str) {
    log::debug!("{}", message);
}

/// 记录警告级别日志
///
/// 这个函数记录警告级别的日志消息，同时输出到日志文件和标准输出。
///
/// # 参数
/// * `message` - 要记录的日志消息
///
/// # 示例
///
/// ```
/// use rsocr::warn;
///
/// warn("磁盘空间不足");
/// ```
pub fn warn(message: &str) {
    log::warn!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Log;
    use std::env;
    use std::fs;

    #[test]
    fn test_log_path() {
        let path = RsOcrLogger::log_path();
        assert!(path.to_string_lossy().contains(".rsOCR/rsOCR.log"));
    }

    #[test]
    fn test_logger_creation() -> Result<()> {
        // 创建临时目录进行测试
        let temp_dir = env::temp_dir().join("rsocr_log_test");

        // 先清理可能存在的旧目录
        let _ = fs::remove_dir_all(&temp_dir);

        fs::create_dir_all(&temp_dir).map_err(RsOcrError::Io)?;

        // 设置HOME环境变量到临时目录（不安全操作）
        let old_home = env::var("HOME").ok();
        unsafe {
            env::set_var("HOME", &temp_dir);
        }

        // 测试创建日志记录器
        let logger = RsOcrLogger::new();
        assert!(logger.is_ok());

        // 测试日志文件是否创建
        let log_path = RsOcrLogger::log_path();
        assert!(log_path.exists(), "日志文件应该存在: {:?}", log_path);

        // 清理 - 恢复原来的HOME环境变量
        unsafe {
            if let Some(old) = old_home {
                env::set_var("HOME", old);
            } else {
                env::remove_var("HOME");
            }
        }

        // 尝试清理目录，忽略错误
        let _ = fs::remove_dir_all(temp_dir);

        Ok(())
    }

    #[test]
    fn test_log_functions() {
        // 这些函数应该能正常调用而不崩溃
        info("测试信息日志");
        error("测试错误日志");
        debug("测试调试日志");
        warn("测试警告日志");

        // 验证函数被调用（通过观察没有panic）
        // 如果没有panic，测试就通过了
    }

    #[test]
    fn test_log_enabled() -> Result<()> {
        // 创建临时目录进行测试
        let temp_dir = env::temp_dir().join("rsocr_log_test2");
        fs::create_dir_all(&temp_dir).map_err(RsOcrError::Io)?;

        unsafe {
            env::set_var("HOME", &temp_dir);
        }

        let logger = RsOcrLogger::new()?;

        // 测试日志级别过滤
        let metadata_info = log::Metadata::builder()
            .level(log::Level::Info)
            .target("test")
            .build();
        assert!(logger.enabled(&metadata_info));

        let metadata_debug = log::Metadata::builder()
            .level(log::Level::Debug)
            .target("test")
            .build();
        // Debug级别应该被过滤掉（因为设置了Info级别）
        assert!(!logger.enabled(&metadata_debug));

        let metadata_error = log::Metadata::builder()
            .level(log::Level::Error)
            .target("test")
            .build();
        assert!(logger.enabled(&metadata_error));

        unsafe {
            env::remove_var("HOME");
        }
        fs::remove_dir_all(temp_dir).ok();

        Ok(())
    }
}

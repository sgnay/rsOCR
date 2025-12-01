//! rsOCR 库 - 提供 OCR 相关功能
//!
//! 这个库提供了图片处理和 OCR API 调用的功能。
//!
//! # 示例
//!
//! ```no_run
//! use rsocr::{image_to_base64, call_ocr_api};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 将图片转换为 base64
//!     let base64_code = image_to_base64("image.png")?;
//!     
//!     // 调用 OCR API
//!     let ocr_response = call_ocr_api(&base64_code, "http://127.0.0.1:1224/api/ocr")?;
//!     
//!     println!("OCR 结果: {:#?}", ocr_response.extra);
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod config;
pub mod gui;
pub mod logger;
pub mod models;
pub mod ocr_utils; // GUI 模块，用于 GUI 应用程序

// 重新导出主要功能，方便用户使用
pub use cli::Args;
pub use config::Config;
pub use logger::{RsOcrLogger, debug, error, info, warn};
pub use models::{OcrOptions, OcrRequest, OcrResponse, Result, RsOcrError};
pub use ocr_utils::{call_ocr_api, image_to_base64};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 获取库版本
pub fn version() -> &'static str {
    VERSION
}

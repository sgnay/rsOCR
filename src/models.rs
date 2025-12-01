//! 数据模型定义
//!
//! 这个模块包含了rsOCR项目中使用的主要数据结构。

use serde::{Deserialize, Serialize};

/// OCR请求选项
#[derive(Debug, Serialize, Deserialize)]
pub struct OcrOptions {
    #[serde(rename = "data.format")]
    pub data_format: String,
}

/// OCR请求
#[derive(Debug, Serialize, Deserialize)]
pub struct OcrRequest {
    pub base64: String,
    pub options: OcrOptions,
}

/// OCR响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OcrResponse {
    /// OCR识别结果数据
    pub data: Option<String>,

    // 其他可能的字段使用泛型结构以适配不同响应
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// 错误类型
#[derive(Debug, thiserror::Error)]
pub enum RsOcrError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP请求错误: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Base64编码错误: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("配置文件错误: {0}")]
    Config(String),

    #[error("OCR API错误: {0}")]
    OcrApi(String),

    #[error("图片处理错误: {0}")]
    ImageProcessing(String),

    #[error("剪贴板错误: {0}")]
    Clipboard(String),

    #[error("通用错误: {0}")]
    Generic(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, RsOcrError>;

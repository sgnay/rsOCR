use crate::models::{OcrRequest, OcrResponse, Result, RsOcrError};
use base64::{Engine as _, engine::general_purpose};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;

/// 将图片文件转换为base64编码
///
/// # 参数
/// * `image_path` - 图片文件路径
///
/// # 返回
/// * `Result<String>` - 成功时返回base64编码字符串，失败时返回错误
pub fn image_to_base64(image_path: &str) -> Result<String> {
    let mut file = File::open(image_path).map_err(RsOcrError::Io)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(RsOcrError::Io)?;

    let base64_code = general_purpose::STANDARD.encode(&buffer);
    Ok(base64_code)
}

/// 调用OCR API进行文字识别
///
/// # 参数
/// * `base64_code` - 图片的base64编码
/// * `url` - OCR API的URL
///
/// # 返回
/// * `Result<OcrResponse>` - 成功时返回OCR响应，失败时返回错误
pub fn call_ocr_api(base64_code: &str, url: &str) -> Result<OcrResponse> {
    let request_data = OcrRequest {
        base64: base64_code.to_string(),
        options: crate::models::OcrOptions {
            data_format: "text".to_string(),
        },
    };

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .map_err(RsOcrError::Http)?;

    if !response.status().is_success() {
        let status = response.status();
        let error_msg = format!(
            "OCR API请求失败: HTTP状态码 {} ({})",
            status,
            status.canonical_reason().unwrap_or("未知错误")
        );
        return Err(RsOcrError::OcrApi(error_msg));
    }

    let ocr_response: OcrResponse = response.json().map_err(RsOcrError::Http)?;
    Ok(ocr_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_image_to_base64() -> Result<()> {
        // 创建一个临时文件进行测试
        let mut temp_file = NamedTempFile::new().map_err(RsOcrError::Io)?;

        // 写入一些测试数据（模拟图片数据）
        let test_data = b"fake image data for testing";
        temp_file.write_all(test_data).map_err(RsOcrError::Io)?;

        // 获取文件路径
        let file_path = temp_file.path().to_str().unwrap();

        // 测试函数
        let result = image_to_base64(file_path);
        assert!(result.is_ok());

        // 验证base64编码结果
        let base64_result = result.unwrap();
        let decoded = general_purpose::STANDARD
            .decode(&base64_result)
            .map_err(RsOcrError::Base64)?;
        assert_eq!(decoded, test_data);

        Ok(())
    }

    #[test]
    fn test_image_to_base64_file_not_found() {
        // 测试文件不存在的情况
        let result = image_to_base64("/non/existent/path/image.png");
        assert!(result.is_err());
    }
}

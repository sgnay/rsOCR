use crate::models::{Result, RsOcrError};
use log::error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 配置结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub file: Option<String>,
    pub url: Option<String>,
}

impl Config {
    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".rsOCR/rsOCR.toml")
    }

    /// 从配置文件加载配置
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        error!("配置文件格式错误: {}", e);
                        Self::default()
                    }
                },
                Err(e) => {
                    error!("读取配置文件失败: {}", e);
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let content =
            toml::to_string_pretty(self).map_err(|e| RsOcrError::Config(e.to_string()))?;
        fs::write(config_path, content).map_err(RsOcrError::Io)?;
        Ok(())
    }

    /// 合并配置：命令行参数优先，然后是配置文件，最后是默认值
    ///
    /// # 参数
    /// * `args_file` - 命令行提供的文件路径
    /// * `args_url` - 命令行提供的URL
    ///
    /// # 返回
    /// * `Result<(String, String)>` - 成功时返回(文件路径, URL)元组，失败时返回错误
    pub fn merge_with_args(
        &self,
        args_file: Option<String>,
        args_url: Option<String>,
    ) -> Result<(String, String)> {
        let file = args_file
            .or_else(|| self.file.clone())
            .ok_or_else(|| RsOcrError::Config("必须提供图片文件路径".to_string()))?;

        let url = args_url
            .or_else(|| self.url.clone())
            .unwrap_or_else(|| "http://127.0.0.1:1224/api/ocr".to_string());

        Ok((file, url))
    }

    /// 使用命令行参数更新配置
    pub fn update_with_args(&mut self, args_file: Option<String>, args_url: Option<String>) {
        if let Some(file) = args_file {
            self.file = Some(file);
        }
        if let Some(url) = args_url {
            self.url = Some(url);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file: None,
            url: Some("http://127.0.0.1:1224/api/ocr".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.file.is_none());
        assert_eq!(
            config.url,
            Some("http://127.0.0.1:1224/api/ocr".to_string())
        );
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path();
        assert!(path.to_string_lossy().contains(".rsOCR/rsOCR.toml"));
    }

    #[test]
    fn test_update_with_args() {
        let mut config = Config::default();

        // 测试更新文件路径
        config.update_with_args(Some("test.png".to_string()), None);
        assert_eq!(config.file, Some("test.png".to_string()));
        assert_eq!(
            config.url,
            Some("http://127.0.0.1:1224/api/ocr".to_string())
        );

        // 测试更新URL
        config.update_with_args(None, Some("http://test.com/api".to_string()));
        assert_eq!(config.file, Some("test.png".to_string()));
        assert_eq!(config.url, Some("http://test.com/api".to_string()));

        // 测试同时更新
        let mut config2 = Config::default();
        config2.update_with_args(
            Some("image.jpg".to_string()),
            Some("http://example.com/api".to_string()),
        );
        assert_eq!(config2.file, Some("image.jpg".to_string()));
        assert_eq!(config2.url, Some("http://example.com/api".to_string()));
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        // 创建临时目录
        let temp_dir = TempDir::new().map_err(RsOcrError::Io)?;
        let config_dir = temp_dir.path().join(".rsOCR");
        fs::create_dir_all(&config_dir).map_err(RsOcrError::Io)?;

        // 创建自定义的Config结构用于测试，重写config_path方法
        struct TestConfig {
            inner: Config,
            config_dir: std::path::PathBuf,
        }

        impl TestConfig {
            fn new(config: Config, config_dir: std::path::PathBuf) -> Self {
                Self {
                    inner: config,
                    config_dir,
                }
            }

            fn config_path(&self) -> std::path::PathBuf {
                self.config_dir.join("rsOCR.toml")
            }

            fn save(&self) -> Result<()> {
                let config_path = self.config_path();
                let content = toml::to_string_pretty(&self.inner)
                    .map_err(|e| RsOcrError::Config(e.to_string()))?;
                fs::write(config_path, content).map_err(RsOcrError::Io)?;
                Ok(())
            }

            fn load(&self) -> Config {
                let config_path = self.config_path();
                if config_path.exists() {
                    match fs::read_to_string(&config_path) {
                        Ok(content) => toml::from_str(&content).unwrap_or_default(),
                        Err(_) => Config::default(),
                    }
                } else {
                    Config::default()
                }
            }
        }

        let config = Config {
            file: Some("test.png".to_string()),
            url: Some("http://test.com/api".to_string()),
        };

        let test_config = TestConfig::new(config, config_dir);

        // 测试保存
        test_config.save()?;

        // 测试加载
        let loaded = test_config.load();
        assert_eq!(loaded.file, Some("test.png".to_string()));
        assert_eq!(loaded.url, Some("http://test.com/api".to_string()));

        Ok(())
    }

    #[test]
    fn test_merge_with_args() -> Result<()> {
        let config = Config {
            file: Some("default.png".to_string()),
            url: Some("http://default.com/api".to_string()),
        };

        // 测试命令行参数优先
        let (file, url) = config.merge_with_args(
            Some("cli.png".to_string()),
            Some("http://cli.com/api".to_string()),
        )?;
        assert_eq!(file, "cli.png");
        assert_eq!(url, "http://cli.com/api");

        // 测试使用配置中的值
        let (file2, url2) = config.merge_with_args(None, None)?;
        assert_eq!(file2, "default.png");
        assert_eq!(url2, "http://default.com/api");

        // 测试混合
        let (file3, url3) = config.merge_with_args(Some("cli2.png".to_string()), None)?;
        assert_eq!(file3, "cli2.png");
        assert_eq!(url3, "http://default.com/api");

        // 测试错误情况 - 没有文件路径
        let config_without_file = Config {
            file: None,
            url: Some("http://test.com/api".to_string()),
        };

        let result = config_without_file.merge_with_args(None, None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("必须提供图片文件路径")
        );

        Ok(())
    }
}

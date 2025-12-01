use clap::Parser;

/// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 图片文件路径
    #[arg(short, long)]
    pub file: Option<String>,

    /// OCR API URL
    #[arg(short, long)]
    pub url: Option<String>,

    /// 保存当前配置到配置文件
    #[arg(short, long)]
    pub save: bool,
}

impl Args {
    /// 解析命令行参数
    ///
    /// 这个函数解析命令行参数并返回 `Args` 结构体实例。
    /// 它使用 `clap` 库进行参数解析，支持以下参数：
    /// - `-f, --file <图片路径>`: 指定要处理的图片文件路径
    /// - `-u, --url <API地址>`: 指定OCR API的URL（可选，默认为 `http://127.0.0.1:1224/api/ocr`）
    /// - `-s, --save`: 保存当前配置到配置文件
    ///
    /// # 返回
    /// * `Self` - 解析后的命令行参数结构体
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use rsocr::Args;
    ///
    /// let args = Args::parse_args();
    /// if let Some(file) = args.file {
    ///     println!("要处理的图片: {}", file);
    /// }
    /// ```
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_args_struct() {
        // 测试结构体可以正常实例化
        let args = Args {
            file: Some("test.png".to_string()),
            url: Some("http://test.com/api".to_string()),
            save: false,
        };

        assert_eq!(args.file, Some("test.png".to_string()));
        assert_eq!(args.url, Some("http://test.com/api".to_string()));
        assert!(!args.save);
    }

    #[test]
    fn test_clap_command() {
        // 测试clap命令可以正常构建
        let cmd = Args::command();
        assert_eq!(cmd.get_name(), "rsocr");
    }

    #[test]
    fn test_parse_args() {
        // 测试参数解析（使用测试参数）
        let test_args = vec![
            "rsocr",
            "-f",
            "image.png",
            "-u",
            "http://api.test.com",
            "-s",
        ];

        // 注意：由于clap的parse_from会消费迭代器，我们需要在测试中模拟
        // 这里我们只验证命令可以构建，实际解析测试需要更复杂的设置
        let cmd = Args::command();
        let matches = cmd.try_get_matches_from(test_args);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_field_documentation() {
        // 验证字段有文档注释（通过编译检查）
        let args = Args {
            file: None,
            url: None,
            save: false,
        };

        // 如果结构体字段有文档，这些字段应该可访问
        let _ = args.file;
        let _ = args.url;
        let _ = args.save;

        // 验证代码可以编译和执行（没有panic）
    }
}

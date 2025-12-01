# rsOCR - Rust OCR工具

一个基于Rust的OCR（光学字符识别）工具，提供命令行界面和图形用户界面。

## 功能特性

- 🖼️ **图片OCR识别**：支持PNG、JPG、JPEG、BMP、GIF格式
- 🌐 **API集成**：通过HTTP API调用OCR服务
- 🖥️ **图形界面**：使用Slint构建的现代化GUI
- 📋 **剪贴板支持**：一键复制识别结果
- ⚙️ **配置管理**：支持配置文件保存和命令行参数
- 📝 **日志记录**：详细的日志记录系统
- 🧪 **完整测试**：全面的单元测试和文档测试

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/sgnay/rsOCR.git
cd rsOCR

# 构建项目
cargo build --release

# 安装到系统
cargo install --path .
```

### 依赖要求

- Rust 1.70+
- Slint 1.13+
- 系统依赖（Linux）：
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
  
  # Fedora
  sudo dnf install libxcb-devel
  ```

## 使用方法

### 命令行模式

```bash
# 基本使用
rsocr --file image.png

# 指定API URL
rsocr --file image.png --url http://127.0.0.1:1224/api/ocr

# 保存配置
rsocr --file image.png --url http://your-api.com/ocr --save-config

# 查看帮助
rsocr --help
```

### 图形界面模式

```bash
# 启动GUI
rsocr-gui
```

GUI功能：
1. 点击"选择图片"按钮选择图片文件
2. 在API URL输入框中设置OCR API地址
3. 点击"执行OCR"开始识别
4. 识别结果会显示在右侧文本区域
5. 点击"复制结果"将结果复制到剪贴板

## 配置

### 配置文件位置
`~/.rsOCR/rsOCR.toml`

### 配置文件示例
```toml
file = "default.png"
url = "http://127.0.0.1:1224/api/ocr"
```

### 环境变量
```bash
# 设置Slint样式（可选：cosmic, material, fluent, native）
export SLINT_STYLE=material

# 构建时使用指定样式
SLINT_STYLE=material cargo build
```

## 项目结构

```
rsOCR/
├── src/
│   ├── main.rs      # 主程序入口
│   ├── lib.rs       # 库定义
│   ├── cli.rs       # 命令行参数解析
│   ├── config.rs    # 配置管理
│   ├── gui.rs       # 图形用户界面
│   ├── logger.rs    # 日志系统
│   ├── models.rs    # 数据结构和错误类型
│   └── ocr_utils.rs # OCR工具函数
├── ui/
│   └── main.slint   # Slint UI定义
├── examples/
│   └── clipboard_test.rs # 剪贴板测试示例
├── build.rs         # 构建配置
└── Cargo.toml       # 项目配置
```

## 开发

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --test config

# 运行文档测试
cargo test --doc
```

### 代码检查
```bash
# 运行Clippy
cargo clippy --all-targets --all-features

# 代码格式化
cargo fmt
```

### 构建选项
```bash
# 调试构建
cargo build

# 发布构建
cargo build --release

# 使用特定Slint样式构建
SLINT_STYLE=fluent cargo build
```

## API集成

### OCR API要求
项目需要与支持以下JSON格式的OCR API配合使用：

**请求格式：**
```json
{
  "base64": "图片的base64编码",
  "options": {
    "data_format": "text"
  }
}
```

**响应格式：**
```json
{
  "data": "识别出的文本内容"
}
```

### 默认API
默认使用 `http://127.0.0.1:1224/api/ocr`，您可以根据需要修改为其他OCR服务。

## 错误处理

项目使用统一的错误处理系统：

```rust
pub enum RsOcrError {
    Io(std::io::Error),
    Http(reqwest::Error),
    OcrApi(String),
    Config(String),
    Base64(base64::DecodeError),
    Generic(String),
}

pub type Result<T> = std::result::Result<T, RsOcrError>;
```

## 贡献

欢迎提交Issue和Pull Request！

1. Fork项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开Pull Request

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [Slint](https://slint-ui.com/) - 现代化的GUI框架
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP客户端
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [serde](https://github.com/serde-rs/serde) - 序列化框架

## 支持

如有问题或建议，请：
1. 查看 [Issues](https://github.com/sgnay/rsOCR/issues)
2. 提交新的Issue
3. 或通过其他方式联系维护者

use std::env;

fn main() {
    // 从环境变量获取Slint样式，默认为"cosmic"
    let style = env::var("SLINT_STYLE").unwrap_or_else(|_| "cosmic".to_string());

    // 验证样式是否有效
    let valid_styles = ["cosmic", "material", "fluent", "native"];
    if !valid_styles.contains(&style.as_str()) {
        eprintln!(
            "警告: 未知的Slint样式 '{}'，有效选项: {:?}",
            style, valid_styles
        );
        eprintln!("使用默认样式: cosmic");
    }

    println!("使用Slint样式: {}", style);

    let config: slint_build::CompilerConfiguration =
        slint_build::CompilerConfiguration::new().with_style(style);

    slint_build::compile_with_config("ui/main.slint", config).expect("Slint rsOCR构建失败");
}

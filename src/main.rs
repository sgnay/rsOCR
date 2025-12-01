use rsocr::gui;

fn main() {
    // 初始化tracing订阅器来过滤zbus的调试信息
    // 设置环境过滤器，只显示warn和error级别的消息，隐藏zbus的调试信息
    let filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn,error,zbus=warn,info".to_string());

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init();

    // 直接运行GUI
    gui::run();
}

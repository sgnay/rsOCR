//! 测试剪贴板功能
use arboard::Clipboard;

fn main() {
    println!("测试 arboard 剪贴板功能（已启用 wayland-data-control feature）");
    println!("当前会话类型: {}", std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "未知".to_string()));
    
    // 测试剪贴板
    match Clipboard::new() {
        Ok(mut clipboard) => {
            println!("✓ 剪贴板初始化成功");
            
            let test_text = "rsOCR 测试文本";
            match clipboard.set_text(test_text) {
                Ok(_) => println!("✓ 文本复制成功: '{}'", test_text),
                Err(e) => println!("✗ 文本复制失败: {}", e),
            }
        }
        Err(e) => println!("✗ 剪贴板初始化失败: {}", e),
    }
}

slint::include_modules!();

use crate::models::RsOcrError;
use crate::ocr_utils;
use arboard::Clipboard;
use rfd::FileDialog;
use slint::SharedString;

/// 运行OCR GUI应用程序
///
/// 这个函数初始化并运行图形用户界面，提供以下功能：
/// - 选择图片文件
/// - 执行OCR识别
/// - 显示识别结果
/// - 复制结果到剪贴板
///
/// # 示例
///
/// ```no_run
/// use rsocr::gui;
///
/// gui::run();
/// ```
pub fn run() {
    let ui = rsOCR::new().expect("component rsOCR new failed!");

    // 选择图片回调
    let ui_weak1 = ui.as_weak();
    ui.on_select_image(move || {
        let ui = ui_weak1.unwrap();

        if let Some(path) = FileDialog::new()
            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif"])
            .pick_file()
        {
            let path_str = path.to_string_lossy().to_string();
            ui.set_selected_image_path(SharedString::from(&path_str));
            ui.set_status_message(SharedString::from(format!("已选择图片: {}", path_str)));

            // 加载图片并设置到UI
            if let Ok(image) = slint::Image::load_from_path(&path) {
                ui.set_selected_image(image);
            } else {
                let error_msg = format!("无法加载图片: {}", path_str);
                log::error!("{}", error_msg);
                ui.set_status_message(SharedString::from(format!("错误: {}", error_msg)));
            }
        }
    });

    // 执行OCR回调
    let ui_weak2 = ui.as_weak();
    ui.on_perform_ocr(move || {
        let ui = ui_weak2.unwrap();
        let image_path = ui.get_selected_image_path().to_string();
        let api_url = ui.get_api_url().to_string();

        if image_path.is_empty() {
            let error_msg = "请先选择图片";
            log::error!("{}", error_msg);
            ui.set_status_message(SharedString::from(format!("错误: {}", error_msg)));
            return;
        }

        ui.set_processing(true);
        ui.set_status_message(SharedString::from("正在处理图片..."));
        log::info!("开始处理图片: {}", image_path);

        // 在后台线程执行OCR处理
        let ui_weak = ui.as_weak();
        std::thread::spawn(move || {
            // 将图片转换为base64
            let result = match ocr_utils::image_to_base64(&image_path) {
                Ok(base64_code) => {
                    // 调用OCR API
                    match ocr_utils::call_ocr_api(&base64_code, &api_url) {
                        Ok(ocr_response) => {
                            if let Some(data) = ocr_response.data {
                                Ok(data)
                            } else {
                                Err(RsOcrError::OcrApi("OCR API返回空结果".to_string()))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            };

            // 在主线程中更新UI
            let ui_weak_clone = ui_weak.clone();
            slint::invoke_from_event_loop(move || {
                let ui = ui_weak_clone.unwrap();
                match result {
                    Ok(data) => {
                        let data_len = data.len();
                        log::info!("OCR处理成功，识别到 {} 个字符", data_len);
                        ui.set_ocr_result(SharedString::from(data));
                        ui.set_status_message(SharedString::from(format!(
                            "OCR完成，识别到 {} 个字符",
                            data_len
                        )));
                    }
                    Err(e) => {
                        let error_msg = e.to_string();
                        log::error!("OCR处理失败: {}", error_msg);
                        ui.set_status_message(SharedString::from(error_msg));
                    }
                }
                ui.set_processing(false);
            })
            .unwrap();
        });
    });

    // 复制到剪贴板回调
    let ui_weak3 = ui.as_weak();
    ui.on_copy_to_clipboard(move || {
        let ui = ui_weak3.unwrap();
        let result = ui.get_ocr_result().to_string();

        if result.is_empty() {
            let error_msg = "没有可复制的内容";
            log::warn!("{}", error_msg);
            ui.set_status_message(SharedString::from(format!("错误: {}", error_msg)));
            return;
        }

        let result_len = result.len();
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(result) {
                    let error_msg = format!("复制失败: {}", e);
                    log::error!("{}", error_msg);
                    ui.set_status_message(SharedString::from(error_msg));
                } else {
                    log::info!("结果已复制到剪贴板，字符数: {}", result_len);
                    ui.set_status_message(SharedString::from("结果已复制到剪贴板"));
                }
            }
            Err(e) => {
                let error_msg = format!("剪贴板初始化失败: {}", e);
                log::error!("{}", error_msg);
                ui.set_status_message(SharedString::from(error_msg));
            }
        }
    });

    ui.run().expect("component rsOCR run failed!");
}

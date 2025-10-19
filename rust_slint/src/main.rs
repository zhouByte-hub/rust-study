// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::{PhysicalPosition, PhysicalSize, WindowPosition, WindowSize};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
    // 设置窗口大小
    let window_width = 800.0;
    let window_height = 800.0;
    ui.window()
        .set_size(WindowSize::Physical(PhysicalSize::new(window_width as u32, window_height as u32)));
    
    // 尝试获取屏幕尺寸并计算居中位置
    // 使用一个常见的屏幕分辨率作为默认值，如果无法获取实际屏幕尺寸
    let default_screen_width = 1920.0;
    let default_screen_height = 1080.0;
    
    // 计算居中位置
    let center_x = (default_screen_width - window_width) / 2.0;
    let center_y = (default_screen_height - window_height) / 2.0;
    
    ui.window()
        .set_position(WindowPosition::Physical(PhysicalPosition::new(center_x as i32, center_y as i32)));
    

    ui.on_request_increase_value({
        // 创建一个对 UI 的弱引用,这是为了避免循环引用，防止内存泄漏,弱引用不会增加引用计数，允许 UI 在必要时被正确释放
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}

// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::{PhysicalPosition, PhysicalSize, WindowPosition, WindowSize};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.window()
        .set_size(WindowSize::Physical(PhysicalSize::new(800, 800)));
    ui.window()
        .set_position(WindowPosition::Physical(PhysicalPosition::new(100, 100)));

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

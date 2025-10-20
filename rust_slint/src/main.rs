// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use display_info::DisplayInfo;
use slint::{PhysicalPosition, PhysicalSize, WindowPosition, WindowSize};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
    // 设置窗口大小
    let window_width = 800.0;
    let window_height = 800.0;
    ui.window()
        .set_size(WindowSize::Physical(PhysicalSize::new(window_width as u32, window_height as u32)));
    
    let (x, y) = {
        let display_info = DisplayInfo::all().unwrap();
        let primary_display = display_info.iter().filter(|item| item.is_primary).next().unwrap();
        (primary_display.width as f64, primary_display.height as f64)
    };
    
    // 计算居中位置
    let center_x = (x / 2.0 - window_width / 2.0) as i32;
    let center_y = (y / 2.0 - window_height / 2.0) as i32;
    
    ui.window()
        .set_position(WindowPosition::Physical(PhysicalPosition::new(center_x, center_y)));
    

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


#[cfg(test)]
mod window_test{
    use display_info::DisplayInfo;


    #[test]
    fn test(){
        let info_list = DisplayInfo::all().unwrap();
        for item in info_list {
            println!("{:?}", item);
            println!("============================")
        }
    }
}
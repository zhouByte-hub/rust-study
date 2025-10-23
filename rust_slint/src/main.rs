// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use display_info::DisplayInfo;
use slint::{PhysicalPosition, WindowPosition, ComponentHandle};
use crate::home::calendar::get_current_weekday;
pub mod home;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
    let (screen_width, screen_height) = {
        let display_info = DisplayInfo::all().unwrap();
        let primary_display = display_info.iter().filter(|item| item.is_primary).next().unwrap();
        (primary_display.width as f64, primary_display.height as f64)
    };
    
    // 计算居中位置
    let center_x = (screen_width / 2.0 - 400.0 / 2.0) as i32;
    let center_y = (screen_height / 2.0 - 600.0 / 2.0) as i32;
    
    ui.window().set_position(WindowPosition::Physical(PhysicalPosition::new(center_x, center_y)));

    // 设置Calendar组件的当前星期属性
    ui.set_current_weekday(get_current_weekday().into());
    Ok(ui.run()?)
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
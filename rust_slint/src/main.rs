// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use crate::home::calendar::get_current_weekday;
use display_info::DisplayInfo;
use slint::{ComponentHandle, PhysicalPosition, WindowPosition};
pub mod home;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let (screen_width, screen_height) = {
        let display_info = DisplayInfo::all().unwrap();
        let primary_display = display_info
            .iter()
            .filter(|item| item.is_primary)
            .next()
            .unwrap();
        (primary_display.width as f64, primary_display.height as f64)
    };

    // 计算居中位置
    let center_x = (screen_width / 2.0 - 400.0 / 2.0) as i32;
    let center_y = (screen_height / 2.0 - 600.0 / 2.0) as i32;

    ui.window()
        .set_position(WindowPosition::Physical(PhysicalPosition::new(
            center_x, center_y,
        )));
    init(ui.as_weak());
    Ok(ui.run()?)
}


/** 
 *  Weak<AppWindow> 是一个弱引用，它不会阻止组件被销毁。在GUI应用中，这非常重要，因为:
 *      1. 避免循环引用 ：如果回调函数持有组件的强引用，而组件又持有回调函数的引用，就会形成循环引用，导致内存泄漏。
 *      2. 安全访问 ：弱引用允许你在不增加引用计数的情况下访问组件，这意味着组件仍然可以在需要时被正常销毁。
 *      3. 检查组件是否仍然存在 ：通过弱引用，你可以检查组件是否仍然存活，而不是盲目地访问可能已经被销毁的组件。
 *      4. 避免空指针异常 ：如果组件在回调函数中被销毁，使用弱引用可以避免空指针异常。
 *  upgrade
 *      1. 升级为强引用 ：通过调用 upgrade() 方法，你可以将弱引用升级为强引用。如果组件仍然存在，升级成功；如果组件已被销毁，升级失败。
 *      2. 检查组件是否存在 ：升级弱引用后，你可以检查组件是否仍然存在，而不是直接访问可能已经被销毁的组件。
 *      3. 临时强引用 ：转换后的强引用只在作用域内有效，离开作用域后引用计数会自动减少，不会阻止组件被销毁。
 */
fn init(window: slint::Weak<AppWindow>) {
    // 设置Calendar组件的当前星期属性
    if let Some(ui) = window.upgrade() {
        ui.set_current_weekday(get_current_weekday().into());
        ui.global::<Logic>().on_magic_operation(|count| {
            println!("magic_operation: {}", count);
            count + 1
        });
    }
    build_user(window);
}

fn build_user(window: slint::Weak<AppWindow>) {
    if let Some(ui) = window.upgrade() {
        // 创建 User 实例
        let user = User {
            user: "李四".into(),  // 使用 .into() 转换为 Slint 的 SharedString
            age: 25,
            sex: true,  // true 表示女性，false 表示男性
            money: 5000.50,
            address: slint::Color::from_rgb_u8(255, 0, 0),  // 红色
            b: slint::Brush::SolidColor(slint::Color::from_rgb_u8(0, 255, 0)),  // 绿色画笔
            c: Sex::Women,  // 使用 Sex 枚举
        };
        
        // 在移动 user 之前获取需要打印的值
        let user_name = user.user.clone();
        let user_age = user.age;
        let user_sex = user.sex;
        
        // 通过全局组件 A 设置 user
        ui.global::<A>().set_user(user);
        
        // 打印日志确认 User 已创建
        println!("User 创建成功: 姓名={}, 年龄={}, 性别={}", 
                 user_name, 
                 user_age, 
                 if user_sex { "女" } else { "男" });
    }
}

#[cfg(test)]
mod window_test {
    use display_info::DisplayInfo;

    #[test]
    fn test() {
        let info_list = DisplayInfo::all().unwrap();
        for item in info_list {
            println!("{:?}", item);
            println!("============================")
        }
    }
}

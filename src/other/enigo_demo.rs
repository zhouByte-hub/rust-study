/**
 *  enigo = "0.6.1"
 * 
 *  Rust 编程语言中一个名为 Enigo 的 crate（即 Rust 的库/包）的特定版本号。
 *  Enigo 是一个用于模拟用户输入（如键盘按键、鼠标移动、点击等）的跨平台库，常用于自动化测试、GUI 自动化脚本、机器人流程自动化（RPA）等场景。
 * 
 *  功能：
 *      1、模拟鼠标移动、点击、滚轮
 *      2、模拟键盘按键（包括组合键）
 *      3、获取当前鼠标位置（部分平台支持）
 *      4、跨平台支持：Windows、macOS、Linux（X11/Wayland）
 */

#[cfg(test)]
mod enigo_test {
    use enigo::{Enigo, Keyboard, Mouse, Settings};

    #[test]
    fn enigo_mouse_test_1(){
        let mut mouse = Enigo::new(&Settings::default()).unwrap();
        mouse.move_mouse(100, 100, enigo::Coordinate::Abs).unwrap();
    }

    #[test]
    fn enigo_mouse_click(){
        let mut mouse = Enigo::new(&Settings::default()).unwrap();
        mouse.move_mouse(0, 0, enigo::Coordinate::Abs).unwrap();
        mouse.button(enigo::Button::Right, enigo::Direction::Click).unwrap();
        mouse.text("Hello World! here is a lot of text  ❤️").unwrap();
    }
}
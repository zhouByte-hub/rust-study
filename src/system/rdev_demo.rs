/**
 * rdev = "0.5.3"
 *
 * 监听用户的键盘和鼠标输入事件（如按键、点击、移动）。
 * 模拟（生成）键盘按键和鼠标操作（如自动点击、输入文本）。
 */
#[cfg(test)]
mod rdev_test {
    use rdev::{EventType, listen};

    #[test]
    fn test() {
        listen(|event| match event.event_type {
            EventType::KeyPress(key) => {
                println!("KeyPress: {:?}", key);
            }
            EventType::ButtonPress(button) => {
                println!("鼠标按键: {:?}", button);
            }
            _ => {
                println!("其他事件: {:?}", event);
            }
        })
        .unwrap();
    }
}

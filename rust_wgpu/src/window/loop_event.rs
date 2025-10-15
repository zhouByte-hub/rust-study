use winit::{
    event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents},
    window::WindowAttributes,
};

pub fn loop_event_run(event_loop: &ActiveEventLoop) {
    create_window(event_loop);
    monitors(event_loop);
    theme(event_loop);
    listen_drive(event_loop);

    // 这个方法中调用 exit 方法退出事件循环
    // set_control(event_loop);
}

/**
 * 通过 EventLoop 创建一个窗口
 */
fn create_window(event_loop: &ActiveEventLoop) {
    match event_loop.create_window(WindowAttributes::default()) {
        Ok(window) => {
            println!("create_window: {:?}", window);
        }
        Err(err) => {
            println!("create_window error: {:?}", err);
        }
    }
}

/**
 * 返回系统上可用的显示器列表
 */
fn monitors(event_loop: &ActiveEventLoop) {
    // 返回系统上可用的显示器列表
    let monitors = event_loop.available_monitors();
    for monitor in monitors {
        println!("monitor: {:?}", monitor);
    }

    // 获取主显示器的信息，如果没有则返回 None
    let primary_monitor = event_loop.primary_monitor().unwrap();
    println!("primary_monitor: {:?}", primary_monitor);
}

/**
 * 返回当前的系统主题
 */
fn theme(event_loop: &ActiveEventLoop) {
    // 返回当前的系统主题
    let theme = event_loop.system_theme().unwrap();
    println!("theme: {:?}", theme);
}

/**
 * 监听系统驱动事件
 */
fn listen_drive(event_loop: &ActiveEventLoop) {
    /*
     * Always 无论窗口是否处于焦点状态，始终报告设备事件。
     * Never 从不捕获设备事件。
     * WhenFocused 仅在窗口获得焦点时报告设备事件。
     */
    event_loop.listen_device_events(DeviceEvents::WhenFocused);
}

/**
 * 事件循环的控制流
 */
#[allow(dead_code)]
fn set_control(event_loop: &ActiveEventLoop) {
    // 设置事件循环的控制流为 Poll
    event_loop.set_control_flow(ControlFlow::Poll);

    // 获取当前的控制流
    let control_flow = event_loop.control_flow();
    println!("control_flow: {:?}", control_flow);

    // 退出事件循环
    event_loop.exit();

    // 检查事件循环是否正在退出
    let exiting = event_loop.exiting();
    println!("exiting: {:?}", exiting);
}

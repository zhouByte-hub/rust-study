pub(crate) mod common;
pub(crate) mod ui;
pub(crate) mod window;

use crate::{
    common::wgpu_test::WgpuApp,
    window::{send_event::CustomEvent},
};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    // let event_loop = EventLoop::new().unwrap();
    let event_loop = EventLoop::<Box<dyn CustomEvent>>::with_user_event()
        .build()
        .unwrap();
    /*  ControlFlow 事件控制流循环规则
           1. Poll：立即处理所有待处理事件，不休眠，常见场景：高优先级动画渲染、游戏循环
           2. Wait：等待新事件到达再处理，普通GUI应用，降低CPU占用
           3. WaitUntil：等待指定时间或新事件到达，常见场景：定时任务、周期性更新
    */
    // event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    // event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(1000)));

    // let mut app = App::default();
    let mut app = WgpuApp::default();
    // 初始化事件循环代理
    // app.init_proxy(event_loop.create_proxy());
    // 在主线程上运行事件循环的应用程序
    event_loop.run_app(&mut app).unwrap();
}

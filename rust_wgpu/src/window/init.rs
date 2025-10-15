use std::thread;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window, WindowId, Theme};

use crate::window::send_event::{CustomEvent, TestApp};
use crate::window::loop_event::loop_event_run;

/**
 * 定义一个结构体用来存放窗口对象以及其他公共对象
 */
#[derive(Default)]
pub struct App {
    window: Option<Window>,
    proxy: Option<EventLoopProxy<Box<dyn CustomEvent>>>,
}

impl App {
    /**
     * 初始化事件循环代理
     */
    pub fn init_proxy(&mut self, proxy: EventLoopProxy<Box<dyn CustomEvent>>) {
        self.proxy = Some(proxy);
    }
}

impl ApplicationHandler<Box<dyn CustomEvent>> for App {

    /**
     * 应用程序恢复运行时调用
     *  1. 在移动设备上，当应用程序从后台切换到前台时
     *  2. 在桌面设备上，当应用程序从最小化状态恢复时
     *  3. 当用户从最小化状态恢复窗口时
     */
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            //设置窗口标题
            .with_title("RUST-STUDY:WGPU:WINIT")
            //设置窗口主题为暗色
            .with_theme(Some(Theme::Dark));
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    /**
     * 窗口事件处理函数
     */
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // 通过 WindowEvent 绑定事件处理逻辑
        match event {
            // 关闭请求事件：当用户点击窗口的关闭按钮、使用键盘快捷键或通过操作系统菜单请求关闭窗口时触发
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            // 重绘请求事件：当窗口需要重新绘制时触发，例如首次显示、调整大小或被其他窗口遮挡后恢复可见
            WindowEvent::RedrawRequested => {
                // self.window.as_ref().unwrap().request_redraw();
            },
            // 窗口大小变化事件：当窗口尺寸被改变时触发，提供新的窗口大小
            WindowEvent::Resized(size) => {
                println!("The window was resized to {:?}", size);
                /* 尺寸单位
                 *  LogicalInsets：表示窗口的内部边距（内边距：窗口内容与窗口边框之间的距离）的逻辑尺寸,逻辑尺寸会根据显示器的 DPI（每英寸点数）进行缩放。
                 *  LogicalPosition：表示窗口或光标的逻辑位置坐标，使用逻辑单位（如逻辑像素）表示位置，逻辑位置会根据窗口的位置和缩放因子进行调整。
                 *  LogicalSize：表示窗口或组件的逻辑尺寸，使用逻辑单位（如逻辑像素）表示尺寸，逻辑尺寸会根据显示器的 DPI（每英寸点数）进行缩放。
                 *  LogicalUnit：定义逻辑测量单位的基础类型，通常是与设备无关的单位，如逻辑像素
                 * 
                 *  PhysicalInsets：表示窗口内部边距的物理尺寸，使用实际物理像素单位，不考虑 DPI 缩放，适用于需要精确像素控制的场景。
                 *  PhysicalPosition：表示窗口或光标的物理位置坐标，使用物理像素单位，不受 DPI 缩放影响。
                 *  PhysicalSize：表示窗口或元素的物理尺寸（宽度和高度），使用实际物理像素单位。
                 *  PhysicalUnit：定义物理测量单位的基础类型，通常是实际物理像素单位。
                 */
                self.window.as_ref().unwrap().set_max_inner_size(Some(LogicalSize::new(500.0, 400.0)));
            },
            // 窗口焦点变化事件：当窗口获得或失去键盘焦点时触发
            WindowEvent::Focused(focused) => {
                println!("The window focus changed to {:?}", focused);
            },
            // 键盘修饰键变化事件：当Shift、Ctrl、Alt等修饰键的状态改变时触发
            WindowEvent::ModifiersChanged(modifiers) => {
                println!("The modifiers changed to {:?}", modifiers);
            },
            // 鼠标光标移动事件：当鼠标光标在窗口内移动时触发，提供当前位置坐标
            WindowEvent::CursorMoved { position, .. } => {
                println!("The cursor moved to {:?}", position);
            },
            // 鼠标光标进入事件：当鼠标光标从窗口外部进入窗口区域时触发
            WindowEvent::CursorEntered { .. } => {
                println!("The cursor entered the window");
            },
            // 鼠标光标离开事件：当鼠标光标从窗口内部离开窗口区域时触发
            WindowEvent::CursorLeft { .. } => {
                println!("The cursor left the window");
            },
            // 鼠标滚轮滚动事件：当用户滚动鼠标滚轮时触发，提供滚动量和滚动阶段
            WindowEvent::MouseWheel { delta, phase, .. } => {
                println!("The mouse wheel scrolled {:?} in phase {:?}", delta, phase);
            },
            // 鼠标按钮输入事件：当鼠标按钮被按下或释放时触发，提供按钮类型和状态
            WindowEvent::MouseInput { state, button, .. } => {
                println!("The mouse {:?} button was {:?}", button, state);
            },
            // 触摸事件：在支持触摸的设备上，当检测到触摸输入时触发
            WindowEvent::Touch(touch) => {
                println!("The touch event {:?}", touch);
            },
            // 键盘输入事件：当键盘按键被按下或释放时触发，提供输入事件详情和设备信息
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                println!("The keyboard input {:?} from device {:?} is synthetic: {:?}", event, device_id, is_synthetic);
            },
            // 激活令牌完成事件：当窗口激活令牌处理完成时触发，用于处理窗口激活相关的权限
            WindowEvent::ActivationTokenDone { serial, token } => {
                println!("The activation token {:?} for serial {:?} was processed", token, serial);
            },
            // 窗口位置移动事件：当窗口在屏幕上的位置改变时触发
            WindowEvent::Moved(position) => {
                println!("The window was moved to {:?}", position);
            },
            // 文件拖放事件：当文件被拖放到窗口上并释放时触发
            WindowEvent::DroppedFile(path) => {
                println!("The file {:?} was dropped on the window", path);
            },
            // 文件悬停事件：当文件被拖到窗口上方但尚未释放时触发
            WindowEvent::HoveredFile(path) => {
                println!("The file {:?} is being hovered over the window", path);
            },
            // 文件悬停取消事件：当文件拖放操作被取消时触发
            WindowEvent::HoveredFileCancelled => {
                println!("The hovered file was cancelled");
            },
            // 窗口销毁事件：当窗口被完全销毁时触发，通常在窗口关闭过程的最后阶段
            WindowEvent::Destroyed => {
                println!("The window was destroyed");
            },
            // IME事件：当输入法编辑器(IME)状态改变时触发，用于处理非拉丁文字输入
            WindowEvent::Ime(ime) => {
                println!("The IME event {:?}", ime);
            },
            // 主题变化事件：当系统主题（如亮色/暗色模式）改变时触发
            WindowEvent::ThemeChanged(theme) => {
                println!("The theme changed to {:?}", theme);
            },
            // 捏合手势事件：当检测到捏合手势时触发，通常用于缩放操作
            WindowEvent::PinchGesture { device_id, delta, phase } => {
                println!("The pinch gesture {:?} from device {:?} with delta {:?}", phase, device_id, delta);
            },
            // 平移手势事件：当检测到平移手势时触发，通常用于移动操作
            WindowEvent::PanGesture { device_id, delta, phase } => {
                println!("The pan gesture {:?} from device {:?} with delta {:?}", phase, device_id, delta);
            },
            // 双击手势事件：当检测到双击手势时触发
            WindowEvent::DoubleTapGesture { device_id } => {
                println!("The double tap from device {:?}", device_id);
            },
            // 旋转手势事件：当检测到旋转手势时触发，通常用于旋转操作
            WindowEvent::RotationGesture { device_id, delta, phase } => {
                println!("The rotation gesture {:?} from device {:?} with delta {:?}", phase, device_id, delta);
            },
            // 触摸板压力事件：当检测到触摸板压力变化时触发
            WindowEvent::TouchpadPressure { device_id, pressure, stage } => {
                println!("The touchpad pressure {:?} from device {:?} on stage {:?}", pressure, device_id, stage);
            },
            // 轴运动事件：当检测到游戏控制器或其他输入设备的轴运动时触发
            WindowEvent::AxisMotion { device_id, axis, value } => {
                println!("The axis {:?} on device {:?} moved to {:?}", axis, device_id, value);
            },
            // 缩放因子变化事件：当窗口的DPI缩放因子改变时触发
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {
                println!("Scale factor changed to {:?} - {:?}", scale_factor, inner_size_writer);
            },
            // 遮挡状态事件：当窗口被其他窗口完全遮挡或不再被遮挡时触发
            WindowEvent::Occluded(occluded) => {
                println!("The window was occluded: {:?}", occluded);
            },
        }
    }

    /**
     * 在事件循环开始处理新的一批事件之前被调用。
     */
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        // println!("new_events: {:?}", cause);
    }

    /**
     * 当通过 EventLoopProxy 发送一个自定义用户事件时被调用。
     */
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Box<dyn CustomEvent>) {
        loop_event_run(event_loop);
        // 调用事件的 run 方法
        event.run();
        println!("user_event: {:?}", event_loop);
    }

    /**
     * 当底层输入设备（如键盘、鼠标、触摸板、游戏手柄等）产生事件时被调用。
     */
    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        println!("device_event: {:?} - {:?} - {:?}", event_loop, device_id, event);
    }

    /**
     * 当应用程序被挂起（如切换到其他应用程序）时被调用。
     */
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("suspended: {:?}", event_loop);
    }

    /**
     * 在事件循环等待新事件之前被调用。
     */
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        println!("about_to_wait: {:?}", event_loop);
        // 使用存储的代理发送事件
        // if let Some(ref proxy) = self.proxy {
        //     proxy.send_event(Box::new(TestApp::new())).unwrap();
        // }else{
        //     println!("about_to_wait: {:?} - proxy is None", event_loop);
        // }
        // thread::sleep(Duration::from_secs(10));
    }

    /**
     * 当应用程序被退出时被调用。
     */
    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("exiting: {:?}", event_loop);
    }

    /**
     * 当应用程序收到内存警告时被调用。
     */
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        println!("memory_warning: {:?}", event_loop);
    }

}

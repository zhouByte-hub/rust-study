use crate::CustomEvent;
use std::sync::Arc;
use wgpu::{
    Device, InstanceDescriptor, PowerPreference, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration,
};
use winit::event_loop::EventLoop;
use winit::window::Theme;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Position},
    window::Window,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct WgpuApp {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    size_change: bool,
}


impl WgpuApp {
    
    pub fn new(event_loop: &EventLoop<Box<dyn CustomEvent>>) -> Self {
        let window_with = 800;
        let window_height = 600;
        // // 通过获取主屏幕来让窗口居中显示
        // let (x, y) = {
        // let primary_monitor = event_loop.primary_monitor().unwrap();
        // let monitor_size = primary_monitor.size();
        // (
        //     (monitor_size.width / 2 - window_with / 2) as i32,
        //     (monitor_size.height / 2 - window_height / 2) as i32,
        // )
        // };
        let attributes = Window::default_attributes()
            //设置窗口标题
            .with_title("RUST-STUDY:WGPU:WINIT")
            //设置窗口主题为暗色
            .with_theme(Some(Theme::Dark))
            .with_position(Position::Physical(PhysicalPosition::new(500, 500)))
            //设置窗口初始大小
            .with_inner_size(LogicalSize::new(window_with, window_height));
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        // 使用 pollster 来在同步上下文中执行异步操作
        // 请求适配器，适配器代表一个物理GPU设备
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            // 电源偏好设置：默认值，平衡性能和功耗
            power_preference: PowerPreference::default(),
            // 指定与此适配器兼容的surface，确保适配器支持我们的渲染目标
            compatible_surface: Some(&surface),
            // 是否强制使用回退适配器（软件渲染），false表示优先使用硬件加速
            force_fallback_adapter: false,
        }))
        .unwrap();

        // 获取设备和队列
        // 设备(Device)是与GPU交互的主要接口，队列(Queue)用于提交命令到GPU
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            // 所需的GPU功能特性，empty()表示不需要特殊功能
            required_features: wgpu::Features::empty(),
            // 设备资源限制，使用默认限制
            required_limits: wgpu::Limits::default(),
            // 设备的调试标签，用于调试时识别设备
            label: None,
            // 内存使用提示，影响GPU内存分配策略
            memory_hints: wgpu::MemoryHints::default(),
            // 实验性功能，disabled()表示不启用任何实验性功能
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            // 跟踪设置，Off表示不启用GPU命令跟踪
            trace: wgpu::Trace::Off,
        }))
        .unwrap();

        // tokio 表示block
        // let (device, queue) = tokio::task::block_in_place(|| {
        //     tokio::runtime::Handle::current().block_on(adapter.request_device(
        //         &wgpu::DeviceDescriptor {
        //             // ... 其他配置保持不变
        //         },
        //     ))
        // }).unwrap();

        // 配置surface
        // Surface配置定义了渲染目标的属性和行为
        let config = wgpu::SurfaceConfiguration {
            // 纹理用途：RENDER_ATTACHMENT表示此surface将用作渲染目标
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // 表面格式：从适配器支持的格式中选择第一个，通常是RGBA8或BGRA8
            format: surface.get_capabilities(&adapter).formats[0],
            // 表面宽度（像素）
            width: window_with,
            // 表面高度（像素）
            height: window_height,
            // 呈现模式：Fifo模式是垂直同步模式，确保画面不会撕裂
            present_mode: wgpu::PresentMode::Fifo,
            // 透明度模式：Auto让系统自动选择合适的透明度处理方式
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            // 视图格式：额外的纹理格式视图，这里为空表示使用默认格式
            view_formats: vec![],
            // 最大帧延迟：设置GPU可以提前准备的最大帧数，2表示可以双缓冲
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size: PhysicalSize::new(window_with, window_height),
            size_change: true,
        }
    }
}

impl ApplicationHandler<Box<dyn CustomEvent>> for WgpuApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("{:?}", event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        println!("{:?}-{:?}", window_id, event);
    }
}

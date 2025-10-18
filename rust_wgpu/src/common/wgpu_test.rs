use crate::CustomEvent;
use image::ImageReader;
use std::sync::Arc;
use wgpu::wgt::{SamplerDescriptor, TextureDescriptor};
use wgpu::{
    AddressMode, Device, Extent3d, InstanceDescriptor, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureViewDescriptor,
};

use winit::event::WindowEvent;
use winit::window::Theme;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Position},
    window::Window,
};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WgpuApp {
    window: Option<Arc<Window>>,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    queue: Option<Queue>,
    config: Option<SurfaceConfiguration>,
    size: PhysicalSize<u32>,
}

impl ApplicationHandler<Box<dyn CustomEvent>> for WgpuApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_with = 800;
        let window_height = 600;
        // 通过获取主屏幕来让窗口居中显示
        let (x, y) = {
            let primary_monitor = event_loop.primary_monitor().unwrap();
            let monitor_size = primary_monitor.size();
            (
                (monitor_size.width / 2 - window_with / 2) as i32,
                (monitor_size.height / 2 - window_height / 2) as i32,
            )
        };
        let attributes = Window::default_attributes()
            //设置窗口标题
            .with_title("RUST-STUDY:WGPU:WINIT")
            //设置窗口主题为暗色
            .with_theme(Some(Theme::Dark))
            .with_position(Position::Physical(PhysicalPosition::new(x, y)))
            //设置窗口初始大小
            .with_inner_size(LogicalSize::new(window_with, window_height));
        self.window = Some(Arc::new(event_loop.create_window(attributes).unwrap()));

        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        self.surface = Some(
            instance
                .create_surface(self.window.as_ref().unwrap().clone())
                .unwrap(),
        );

        // 使用 pollster 来在同步上下文中执行异步操作
        // 请求适配器，适配器代表一个物理GPU设备
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            // 电源偏好设置：默认值，平衡性能和功耗
            power_preference: PowerPreference::default(),
            // 指定与此适配器兼容的surface，确保适配器支持我们的渲染目标
            compatible_surface: Some(self.surface.as_ref().unwrap()),
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
        // Surface配置定义了渲染目标的属性和行为，需要确保 SurfaceTexture 的宽高不能为 0，这会导致你的应用程序崩溃。
        let config = wgpu::SurfaceConfiguration {
            // 纹理用途：RENDER_ATTACHMENT表示此surface将用作渲染目标
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // 表面格式：从适配器支持的格式中选择第一个，通常是RGBA8或BGRA8
            format: self
                .surface
                .as_ref()
                .unwrap()
                .get_capabilities(&adapter)
                .formats[0],
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

        self.surface.as_ref().unwrap().configure(&device, &config);

        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
        self.size = PhysicalSize::new(window_with, window_height);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            // 窗口大小发生变化，画布也需要发生变化
            WindowEvent::Resized(size) => {
                self.size.width = size.width;
                self.size.height = size.height;
                self.surface.as_ref().unwrap().configure(
                    self.device.as_ref().unwrap(),
                    &self.config.as_ref().unwrap(),
                );
                println!("{:?}", self.size);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // 获取当前可用的surface纹理，这将是我们的渲染目标
                // get_current_texture()返回一个SurfaceTexture，包含GPU可以写入的纹理
                let output = self
                    .surface
                    .as_ref()
                    .unwrap()
                    .get_current_texture()
                    .unwrap();

                // 创建纹理视图，这是渲染管道实际渲染到的目标
                // 纹理视图定义了如何解释纹理数据（如格式、范围等）
                let view = output
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                // 创建命令编码器，用于记录GPU命令
                // 命令编码器允许我们构建一系列GPU操作，然后一次性提交到GPU队列
                let mut encoder = self.device.as_ref().unwrap().create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"), // 调试标签，便于GPU调试工具识别
                    },
                );

                // 开始渲染通道，这是实际渲染操作的开始
                // 渲染通道定义了渲染目标的状态和如何处理这些目标
                encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"), // 调试标签
                    // 颜色附件定义了渲染目标的颜色缓冲区
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,          // 使用我们之前创建的纹理视图作为渲染目标
                        resolve_target: None, // 多重采样解析目标，None表示不使用多重采样
                        depth_slice: None,    // 3D纹理的深度切片，2D纹理不需要
                        ops: Operations {
                            // 加载操作：清除为白色，表示渲染开始时将缓冲区清空为白色
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            // 存储操作：存储渲染结果，表示渲染结束后保留缓冲区内容
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default() // 使用默认值填充其他字段（如深度/模板附件）
                });

                // 将编码的命令提交到GPU队列执行
                // encoder.finish()完成命令记录，submit()将命令发送到GPU执行
                self.queue.as_ref().unwrap().submit(Some(encoder.finish()));

                // 将渲染结果呈现到屏幕
                // present()操作会将渲染的纹理显示在窗口上，完成一帧的渲染
                output.present();
                render_images(self.device.as_ref().unwrap(), self.queue.as_ref().unwrap());
            }
            _ => {}
        }
    }
}

fn render_images(device: &Device, queue: &Queue) {
    // 使用绝对路径加载图片，确保无论从哪个目录运行程序都能找到文件
    let image_path = format!("{}/src/common/1.png", env!("CARGO_MANIFEST_DIR"));
    let image = ImageReader::open(image_path).unwrap().decode().unwrap();

    // 定义纹理尺寸
    let size = Extent3d {
        width: image.width(),
        height: image.height(),
        depth_or_array_layers: 1,
    };

    // 创建GPU纹理对象
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("Image Texture"), // 调试标签，便于识别
        size: size.clone(),
        mip_level_count: 1,                    // Mipmap级别数量，1表示只有基础级别
        sample_count: 1,                       // 采样次数，1表示不使用多重采样
        dimension: wgpu::TextureDimension::D2, // 2D纹理
        format: wgpu::TextureFormat::Rgba8UnormSrgb, // RGBA格式，8位无符号归一化，sRGB色彩空间
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, // 用途：可作为纹理绑定和复制目标
        view_formats: &[], // 额外的视图格式，空数组表示只使用主格式
    });

    // 将图像数据复制到GPU纹理中
    queue.write_texture(
        TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,                     // 使用基础mipmap级别（从0开始）
            origin: wgpu::Origin3d::ZERO,     // 从纹理原点开始写入
            aspect: wgpu::TextureAspect::All, // 复制所有方面（颜色、深度等）
        },
        &image.to_rgba8(), // 将图像转换为RGBA8格式
        TexelCopyBufferLayout {
            offset: 0,                           // 缓冲区偏移量
            bytes_per_row: Some(size.width * 4), // 每行字节数（RGBA每像素4字节）
            rows_per_image: Some(size.height),   // 每个图像的行数
        },
        size.clone(), // 复制区域大小
    );
    let view = texture.create_view(&TextureViewDescriptor::default());
    let diffuse_sampler = device.create_sampler(&SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
}

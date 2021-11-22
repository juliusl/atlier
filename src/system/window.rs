use futures::executor::block_on;
use winit::dpi::{LogicalSize};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct WindowContext {
    pub event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub instance: Option<wgpu::Instance>,
    pub window: Option<winit::window::Window>,
    pub physical_size: Option<winit::dpi::PhysicalSize<u32>>,
    pub surface: Option<wgpu::Surface>,
    pub hidpi_scale_factor: Option<f64>,
    pub font_size: Option<f32>,
}

impl Default for WindowContext {
    fn default() -> Self {
        WindowContext{
            event_loop: None,
            instance: None,
            window: None,
            physical_size: None,
            surface: None,
            hidpi_scale_factor: None,
            font_size: None
        }
    }
}

impl WindowContext {
    pub fn new(title: &str, width: f64, height: f64) -> Self {
        let event_loop = EventLoop::new();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let (window, size, surface, hidpi_factor, font_size) = {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(LogicalSize {
                width: width,
                height: height,
            });
            window.set_title(&title);
            let size = window.inner_size();

            window.set_maximized(true);
            let surface = unsafe { instance.create_surface(&window) };
            let hidpi_factor = window.scale_factor();
            let font_size = (16.0 * hidpi_factor) as f32;

            (window, size, surface, hidpi_factor, font_size)
        };

        Self {
            event_loop: Some(event_loop),
            instance: Some(instance),
            window: Some(window),
            physical_size: Some(size),
            surface: Some(surface),
            hidpi_scale_factor: Some(hidpi_factor),
            font_size: Some(font_size),
        }
    }
}

pub struct Hardware {
    pub window_context: WindowContext,
    pub adapter: Option<wgpu::Adapter>,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub surface_desc: Option<wgpu::SurfaceConfiguration>,
}

impl From<WindowContext> for Hardware {
    fn from(context: WindowContext) -> Self {
        let hardware = move || {
            if let (Some(instance), Some(surface), Some(physical_size)) = (context.instance, context.surface, context.physical_size) {
                let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    ..Default::default()
                }))
                .unwrap();
        
                let (device, queue) =
                    block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();
        
                let surface_descriptor = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: physical_size.width,
                    height: physical_size.height,
                    present_mode: wgpu::PresentMode::Fifo,
                };
        
                return Hardware {
                    window_context: WindowContext {
                        instance: Some(instance),
                        surface: Some(surface),
                        physical_size: Some(physical_size),
                        event_loop: context.event_loop,
                        window: context.window,
                        hidpi_scale_factor: context.hidpi_scale_factor,
                        font_size: context.font_size
                    },
                    adapter: Some(adapter),
                    device: Some(device),
                    queue: Some(queue),
                    surface_desc: Some(surface_descriptor),
                }
            } else {
                panic!("Could not initialize hardware")
            }
        };

        hardware()
    }
}

impl Default for Hardware {
    fn default() -> Self {
        Self {
            window_context: WindowContext::default(),
            adapter: None,
            device: None,
            queue: None,
            surface_desc: None
        }
    }
}
mod window;
mod gui;
mod node;

use window::WindowContext;
use window::Hardware;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use imgui::FontSource;

pub use gui::GUI;
pub use gui::GUIUpdate;
pub use gui::ControlState;

pub use node::NodeModule;
pub use node::NodeApp;
pub use node::NodeResource;
pub use node::EditorResource;
pub use node::Sum;
pub use node::AttributeValue;

pub trait App<'a> {
    fn get_window(&self) -> imgui::Window<'static, String>;
    fn show(&mut self, ui: &imgui::Ui);
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Int(i32),
    Bool(bool),
    FloatRange(f64, f64, f64),
    IntRange(i64, i64, i64),
    TextBuffer(String),
}

impl Into<AttributeValue> for Value {
    fn into(self) -> AttributeValue {
        AttributeValue::System(self)
    }
}

pub fn new_gui_system<'a, A>(title: &str, width: f64, height: f64, apps: Vec<A>) ->  (winit::event_loop::EventLoop<()>, GUI<A>) {
    let window_context = window::WindowContext::new(title, width, height);
    let setup = move || {
        if let Hardware {
            window_context: WindowContext{
                surface: Some(surface),
                window: Some(window),
                hidpi_scale_factor: Some(hidpi_scale_factor),
                font_size: Some(font_size),
                event_loop: Some(event_loop),
                instance: Some(instance),
                physical_size: Some(physical_size)
            },
            device: Some(device),
            queue: Some(queue),
            surface_desc: Some(surface_desc),
            adapter: Some(adapater),
        } = Hardware::from(window_context) {
            surface.configure(&device, &surface_desc);
            // Set up dear imgui
            let mut imgui = imgui::Context::create();

            let setup_imgui = &mut imgui;

            let mut platform = imgui_winit_support::WinitPlatform::init(setup_imgui);
            platform.attach_window(
                setup_imgui.io_mut(),
                &window,
                imgui_winit_support::HiDpiMode::Default,
            );
            setup_imgui.set_ini_filename(None);
        
            setup_imgui.io_mut().font_global_scale = (1.0 / hidpi_scale_factor) as f32;
        
            setup_imgui.fonts().add_font(&[FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

            let renderer_config = RendererConfig {
                texture_format: surface_desc.format,
                ..Default::default()
            };
            
            let renderer =Renderer::new(
                setup_imgui, 
                &device, 
                &queue, 
                renderer_config);
            

            let gui = GUI {
                imgui: imgui,
                renderer: renderer,
                instance: instance,
                window: window,
                physical_size: physical_size,
                surface: surface,
                hidpi_scale_factor: hidpi_scale_factor,
                font_size: font_size,
                adapter: adapater,
                device: device,
                queue: queue,
                surface_desc: surface_desc,
                platform: platform,
                last_frame: None,
                last_cursor: None,
                app: apps,
            };

            return (event_loop, gui);
        } else {
            panic!("Could not initialize hardware")
        };
    };

    setup()
}
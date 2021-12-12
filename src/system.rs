mod attribute;
mod font;
mod gui;
mod node;
mod window;
mod state;
mod filesystem;

use std::fs::File;
use std::hash::Hash;
use imgui::FontSource;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use window::Hardware;
use window::WindowContext;

pub use gui::ControlState;
pub use gui::GUIUpdate;
pub use gui::GUI;

pub use node::expression;
pub use node::expression::*;
pub use node::Display;
pub use node::EditorResource;
pub use node::Initializer;
pub use node::Module;
pub use node::NodeEditor;
pub use node::NodeExterior;
pub use node::NodeInterior;
pub use node::NodeModule;
pub use node::NodeResource;
pub use node::NodeVisitor;
pub use node::Output;
pub use node::Reducer;

pub use font::cascadia_code;
pub use font::monaco;
pub use font::segoe_ui;

pub use attribute::Attribute;
pub use attribute::Routine;
pub use attribute::Value;

pub use state::State;

use crate::store::Store;

pub trait App<'a> {
    fn get_window(&self) -> imgui::Window<'static, String>;
    fn show(&mut self, ui: &imgui::Ui);
    // fn update(&mut self);
}

pub trait CoreSystem 
where
    Self: Default + Clone
{
    type Node: Clone + Hash + Default;

    /// `get_store` returns the current store
    fn get_store(&mut self) -> Store<Self::Node>;

    /// `set_store` sets the current store
    fn set_store(&mut self, next: Store<Self::Node>) -> Self; 

    /// `new` initalizes a new system from application `T`
    fn new<T>(app: T) -> Self 
    where 
        Self: Into<Self::Node>,
        T: Into<Self::Node> + Clone,
    {
       let mut new_core_system = Self::default();

       let next = &mut new_core_system.get_store();
       let next = next
        .edge_node(app.clone())
        .edge_link(app.clone(), new_core_system.clone().into());

       new_core_system.clone().set_store(next)
    }

    /// `module` adds an edge node for the module
    fn with<T>(&mut self, module: T) -> Self
    where
        Self: Into<Self::Node>,
        T: Into<Self::Node> + Clone,
    {
        let next = self.get_store();
        let tool_edge = module.clone();
        let next = next.node(self.clone().into()).edge_node(tool_edge);

        let tool_edge = module.clone();
        let next = next.edge_link(tool_edge, self.clone().into());

        self.set_store(next)
    }
    
    /// `on_update` is called after an update has occured
    fn on_update(&self) {}

    /// `update` passes in a fn that returns the next Self and Store
    fn update(&mut self, update: fn(&Self, Store<Self::Node>) -> (Self, Store<Self::Node>)) -> Self {
        let store = self.get_store();

        let (mut next, next_store) = update(&self, store);

        let set = move || next.set_store(next_store);

        let next = set();

        next.on_update();

        next
    }
}

pub fn new_gui_system<'a, A>(
    title: &str,
    width: f64,
    height: f64,
    apps: Vec<A>,
) -> (winit::event_loop::EventLoop<()>, GUI<A>) {
    let window_context = window::WindowContext::new(title, width, height);
    let setup = move || {
        if let Hardware {
            window_context:
                WindowContext {
                    surface: Some(surface),
                    window: Some(window),
                    hidpi_scale_factor: Some(hidpi_scale_factor),
                    font_size: Some(font_size),
                    event_loop: Some(event_loop),
                    instance: Some(instance),
                    physical_size: Some(physical_size),
                },
            device: Some(device),
            queue: Some(queue),
            surface_desc: Some(surface_desc),
            adapter: Some(adapter),
        } = Hardware::from(window_context)
        {
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

            if let Some(cascadia_code) = cascadia_code() {
                setup_imgui.fonts().add_font(&[FontSource::TtfData {
                    data: &cascadia_code,
                    config: Some(imgui::FontConfig {
                        name: Some("Cascadia Code".to_string()),
                        oversample_h: 1,
                        pixel_snap_h: true,
                        size_pixels: font_size,
                        ..Default::default()
                    }),
                    size_pixels: font_size,
                }]);
            }

            if let Some(monaco) = monaco() {
                setup_imgui.fonts().add_font(&[FontSource::TtfData {
                    data: &monaco,
                    config: Some(imgui::FontConfig {
                        name: Some("Monaco".to_string()),
                        oversample_h: 1,
                        pixel_snap_h: true,
                        size_pixels: font_size,
                        ..Default::default()
                    }),
                    size_pixels: font_size,
                }]);
            }

            if let Some(segoe_ui) = segoe_ui() {
                setup_imgui.fonts().add_font(&[FontSource::TtfData {
                    data: &segoe_ui,
                    config: Some(imgui::FontConfig {
                        name: Some("Segoe UI".to_string()),
                        oversample_h: 1,
                        pixel_snap_h: true,
                        size_pixels: font_size,
                        ..Default::default()
                    }),
                    size_pixels: font_size,
                }]);
            }

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

            let renderer = Renderer::new(setup_imgui, &device, &queue, renderer_config);

            let gui = GUI {
                imgui,
                renderer,
                instance,
                window,
                physical_size,
                surface,
                hidpi_scale_factor,
                font_size,
                adapter,
                device,
                queue,
                surface_desc,
                platform,
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

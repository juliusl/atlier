mod font;
mod gui;
mod window;

use imgui::FontSource;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use specs::Builder;
use specs::Component;
use specs::DispatcherBuilder;
use specs::System;
use specs::World;
use specs::WorldExt;
use specs::storage::DenseVecStorage;
use std::any::Any;
use std::hash::Hash;
use window::Hardware;
use window::WindowContext;
use winit::event_loop::ControlFlow;

pub use gui::ControlState;
pub use gui::GUIUpdate;
pub use gui::GUI;

pub use font::cascadia_code;
pub use font::monaco;
pub use font::segoe_ui;

/// The App trait allows an "editor" to be shown
pub trait App: Any + Sized {
    /// name of this app
    fn name() -> &'static str;

    /// default window_size to use for this app
    fn window_size() -> &'static [f32; 2] {
        &[1920.0, 1080.0]
    }

    /// Shows the editor
    fn show_editor(&mut self, ui: &imgui::Ui);
}

/// The Extension trait allows customization of the UI implementation
/// Requires the state to implement specs:Component
pub trait Extension: App {
    /// extend the World by adding additional resources and systems
    fn extend(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'static, 'static>,
    ) -> DispatcherBuilder<'static, 'static>;
}

#[derive(Clone, Default, Debug, Component)]
#[storage(DenseVecStorage)]
pub struct Attribute {
    id: u32,
    name: String,
    value: Value,
}

impl Attribute {
    pub fn new(id: u32, name: String, value: Value) -> Attribute {
        Attribute {
            id,
            name,
            value,
        }
    }

    pub fn get_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    // sets the id/owner of this attribute
    pub fn set_id(&mut self, id: u32) {
        self.id = id; 
    }

    /// read the name of this attribute
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// read the current value of this attribute
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl App for Attribute {
    fn name() -> &'static str {
        "Attribute"
    }

    fn show_editor(&mut self, ui: &imgui::Ui) {
        let label = format!("{}_{:#4x}", self.name, self.id);
        match &mut self.value {
            Value::Empty => { 
                ui.label_text(label, "Empty Attribute");
            },
            Value::Float(float) => {
                ui.input_float(label, float).build();
            },
            Value::Int(int) => {
                ui.input_int(label, int).build();
            },
            Value::Bool(bool) => {
                ui.checkbox(label,  bool);
            },
            Value::FloatRange(f1, f2, f3) => {
                ui.input_float3(label, &mut [*f1, *f2, *f3]).build();
            },
            Value::IntRange(i1, i2, i3) => {
                ui.input_int3(label, &mut [*i1, *i2 ,*i3]).build();
            },
            Value::TextBuffer(text) => {
                ui.input_text(label, text).build();
            },
        };
    }
}

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
pub enum Value {
    Empty,
    Float(f32),
    Int(i32),
    Bool(bool),
    FloatRange(f32, f32, f32),
    IntRange(i32, i32, i32),
    TextBuffer(String),
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Float(f) => f.to_bits().hash(state),
            Value::Int(i) => i.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::FloatRange(f, fm, fmx) => {
                f.to_bits().hash(state);
                fm.to_bits().hash(state);
                fmx.to_bits().hash(state);
            }
            Value::IntRange(i, im, imx) => {
                i.hash(state);
                im.hash(state);
                imx.hash(state);
            }
            Value::TextBuffer(txt) => txt.hash(state),
            Value::Empty => {}
        };
    }
}

pub fn start_editor<A, F>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extend: F
) where
    A: App + for<'a> System<'a> + Send,
    F: 'static + Fn(&mut A, &mut World, &mut DispatcherBuilder)
{
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });
    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    // As part of the gui system setup, the gui system will also begin setup of the application system
    let (event_loop, gui) = new_gui_system(title, width, height, app, extend);

    // Create the specs dispatcher
    let dispatcher = DispatcherBuilder::new();
    let mut dispatcher = dispatcher.with_thread_local(gui).build();
    dispatcher.setup(&mut w);

    // Create a gui entity that we can use to communicate with the window
    let gui_entity = w
        .create_entity()
        .maybe_with(Some(GUIUpdate {
            event: winit::event::Event::Suspended,
        }))
        .build();

    // Starts the event loop
    event_loop.run(move |event, _, control_flow| {
        // Note: We technically only need the thread local systems to be called because we don't 
        // have any par able systems. However if we do add any this next line will need to be uncommented 
        //dispatcher.dispatch_seq(&w);

        // THREAD LOCAL
        // Dispatch the next event to the gui_entity that is rendering windows
        if let Some(event) = event.to_static() {
            if let Err(err) = w
                .write_component()
                .insert(gui_entity, GUIUpdate { event: event })
            {
                println!("Error: {}", err)
            }
            dispatcher.dispatch_thread_local(&w);
        }

        // This cleans up un-used resources in the world
        w.maintain();

        // The gui_system can dispatch back some control state, which we can read here
        let control_state = w.read_resource::<ControlState>();
        if let Some(c) = control_state.control_flow {
            *control_flow = c;
        } else {
            *control_flow = ControlFlow::Poll;
        }
    });
}

pub fn new_gui_system<A, F>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extension: F,
) -> (winit::event_loop::EventLoop<()>, GUI<A, F>)
where
    A: App + System<'static>,
    F: FnOnce(&mut A, &mut World, &mut DispatcherBuilder)
{
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
                window_title: title.to_string(),
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
                app,
                extension,
                app_world: World::new(),
                app_dispatcher: None,
            };

            return (event_loop, gui);
        } else {
            panic!("Could not initialize hardware")
        };
    };

    setup()
}

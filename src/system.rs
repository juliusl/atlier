mod window;
mod gui;
mod node;
mod font;
mod attribute;

use window::WindowContext;
use window::Hardware;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use imgui::FontSource;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;

pub use gui::GUI;
pub use gui::GUIUpdate;
pub use gui::ControlState;

pub use node::NodeModule;
pub use node::NodeEditor;
pub use node::NodeResource;
pub use node::EditorResource;
pub use node::expression;
pub use node::NodeVisitor;
pub use node::NodeInterior;
pub use node::NodeExterior;
pub use node::Reducer;
pub use node::Display;
pub use node::Output;
pub use node::expression::*;

pub use font::cascadia_code;
pub use font::monaco;
pub use font::segoe_ui;

pub use attribute::Attribute;
pub use attribute::Resource;

pub trait App<'a> {
    fn get_window(&self) -> imgui::Window<'static, String>;
    fn show(&mut self, ui: &imgui::Ui);
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Int(i32),
    Bool(bool),
    FloatRange(f32, f32, f32),
    IntRange(i32, i32, i32),
    TextBuffer(String),
}

#[derive(Clone, Hash)]
pub struct State(BTreeMap<String, Attribute>, Option<Vec<Update>>);

#[derive(Clone, Hash)]
pub enum Update {
    Insert(String, Attribute),
    Delete(String),
}

impl Default for State {
    fn default() -> Self {
        Self(BTreeMap::default(), None)
    }
}

impl State {
    pub fn get(&self, str: &'static str) -> Option<Attribute> {
        let State(map, ..) = self.next_state();

        if let Some(v) = map.get(str) {
            Some(v.to_owned())
        } else {
            None
        }
    }

    pub fn get_hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();

        self.hash(&mut hasher);

        hasher.finish()
    }

    // dispatch returns a new state with a new message
    pub fn dispatch(&self, message: Update) -> Self {
        match self {
            State(state, Some(updates)) => {
                let mut next = updates.clone();
                next.push(message);
                State(state.clone(), Some(next))
            }
            State(state, None) => {
                State(state.clone(), Some(vec![message]))
            }
        }
    }

    // next_state flattens any/all messages into a new State 
    pub fn next_state(&self) -> Self {
         let next_state = match self {
            State(state, Some(updates)) => {
                let mut next = state.clone();
                updates.iter().for_each(|u| 
                    match u {
                        Update::Insert(key, value) => { 
                            next.insert(key.clone(), value.clone());
                        }
                        Update::Delete(key) => {
                            next.remove(key);
                        }
                    }
                );

                next
            }
            State(state, None) => {
                state.to_owned()
            }
        };

        State(next_state, None)
    }
}

#[test]
fn test_dispatch() {
    let state = State::default();
    let old = state.get_hash_code();

    let state = state
        .dispatch(Update::Insert("test".to_string(), 10.0.into()))
        .dispatch(Update::Insert("test".to_string(), 14.0.into()))
        .next_state();

    let new = state.get_hash_code();
    assert_ne!(old, new); 

    if let Some(v) = state.get("test") {
        assert_eq!(14.0, v.to_owned().into());
    }

}

impl Into<Attribute> for State {
    fn into(self) -> Attribute {
        Attribute::Map(self.0)
    }
}

impl From<&BTreeMap<String, Attribute>> for State {
    fn from(state: &BTreeMap<String, Attribute>) -> Self {
        State(state.clone(), None)
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
            },
            Value::TextBuffer(txt) => txt.hash(state),
        };
    }
}

impl Into<Attribute> for Value {
    fn into(self) -> Attribute {
        Attribute::Literal(self)
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
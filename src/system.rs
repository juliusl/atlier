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

#[derive(Debug, Clone, Hash)]
pub enum Routines {
    Name(fn() -> &'static str),
    Select(fn(state: State) -> (u64, Option<Attribute>)),
    Reduce(fn(attribute: Option<Attribute>) -> Option<Attribute>),
    Transform(fn(state: State) -> Option<Attribute>),
    Next(fn(state: State) -> Option<State>),
}

impl From<fn() -> &'static str> for Routines {
    fn from(f: fn() -> &'static str) -> Self {
        Routines::Name(f)
    }
}

impl From<fn(state: State) -> (u64, Option<Attribute>)> for Routines {
    fn from(f: fn(state: State) -> (u64, Option<Attribute>)) -> Self {
        Routines::Select(f)
    }
}

impl From<fn(attribute: Option<Attribute>) -> Option<Attribute>> for Routines {
    fn from(f: fn(attribute: Option<Attribute>) -> Option<Attribute>) -> Self {
        Routines::Reduce(f)
    }
}

impl From<fn(state: State) -> Option<Attribute>> for Routines {
    fn from(f: fn(state: State) -> Option<Attribute>) -> Self {
        Routines::Transform(f)
    }
}

impl From<fn(state: State) -> Option<State>> for Routines {
    fn from(f: fn(state: State) -> Option<State>) -> Self {
        Routines::Next(f)
    }
}

#[derive(Clone, Hash)]
pub struct State(BTreeMap<String, Attribute>, Option<Vec<Update>>, Option<BTreeMap<String, String>>);

#[derive(Clone, Hash)]
pub enum Update {
    Namespace(String),
    Assign(String, String),
    Insert(String, Attribute),
    Delete(String),
    Merge(BTreeMap<String, Attribute>),
}

impl Default for State {
    fn default() -> Self {
        Self(BTreeMap::default(), None, None)
    }
}

impl State {
    /// `get` returns the latest version of the attribute
    /// `get` will flatten all messages into a state before getting the next value. This has no side effects on the original collection.
    pub fn get(&self, key: &'static str) -> Option<Attribute> {
        let State(map, ..) = self.next_state();

        if let Some(v) = map.get(key) {
            Some(v.to_owned())
        } else {
            None
        }
    }

    /// `get_hash_code` returns the current hash value for this map
    pub fn get_hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();

        self.hash(&mut hasher);

        hasher.finish()
    }

    /// `dispatch` returns a new state with a new message
    pub fn dispatch(&self, message: Update) -> Self {
        match self {
            State(state, Some(updates), _) => {
                let mut next = updates.clone();
                next.push(message);
                State(state.clone(), Some(next), None)
            }
            State(state, None, _) => {
                State(state.clone(), Some(vec![message]), None)
            }
        }
    }

    /// `snapshot` returns a clone of state as-is w/o updates
    pub fn snapshot(&self) -> Self {
        State(self.0.clone(), None, self.2.clone())
    }

    /// `next_state` flattens any/all updates and returns a new State 
    pub fn next_state(&self) -> Self {
         let next_state = match self {
            State(state, Some(updates), _) => {
                let mut next = state.clone();

                let mut namespace = String::default();

                updates.iter().for_each(|u| 
                    match u {
                        Update::Assign(key, value) => todo!(),
                        Update::Namespace(ns) => {
                            namespace = ns.to_owned();
                        }
                        Update::Insert(key, value) => { 
                            let key = format!("{}/{}", namespace, key);
                            next.insert(key, value.clone());
                        }
                        Update::Delete(key) => {
                            let key = format!("{}/{}", namespace, key);
                            next.remove(&key);
                        }
                        Update::Merge(map) => {
                            for (key, value) in map {
                                let key = format!("{}/{}", namespace, key);
                                next.insert(key, value.to_owned());
                            }
                        }
                    }
                );

                next
            }
            State(state, None, _) => {
                state.to_owned()
            }
        };

        State(next_state, None, None)
    }

    pub fn set_namespace(&self, ns: &str) -> Self {
        self.dispatch(Update::Namespace(ns.to_string()))
    }

    /// `insert` is a helper method to dispatch an insert update
    pub fn insert<V>(&self, key: &str, value: V) -> Self 
    where 
        V: Into<Attribute>
    {
        self.dispatch(Update::Insert(key.to_string(), value.into()))
    }

    /// `merge` is a helper method to dispatch a merge update
    pub fn merge<M>(&self, map: M) -> Self 
    where
        M: Into<BTreeMap<String, Attribute>>
    {
        self.dispatch(Update::Merge(map.into()))
    }

    /// `delete` is a helper method to dispatch a delete update
    pub fn delete(&self, key: &str) -> Self {
        self.dispatch(Update::Delete(key.to_string()))
    }

    /// `map` creates a clone of a subset of parameters from `State`
    pub fn map(&self, parameters: &[&'static str]) -> Self {
        let mut mapped = Self::default();
        parameters
            .iter()
            .map(|p| (p, self.get(p)))
            .filter_map(|p|match p {
                (name, Some(attr)) => Some((name, attr)),
                _ => None,
            })
            .for_each(|(n, a)| {
                mapped = mapped.insert(*n, a);
            });

        mapped
    }

    /// `reduce` takes the current state and creates T
    /// `T` is a Reducer that will reduce/format the parameters into the next State
    pub fn reduce<T>(&self, reducer: fn(state: State, t: T) -> Self) -> Self 
    where
        T: Reducer + From<State>
    {
        reducer(self.snapshot(), T::from(self.clone()))
    }

    /// `visit` allows a visitor to initialize from this state
    pub fn visit<'a, T>(&self) -> T::Visitor
    where
        T: NodeInterior<'a>
    {
        T::accept(self.clone())
    }
}

impl Into<BTreeMap<String, Attribute>> for State {
    fn into(self) -> BTreeMap<String, Attribute> {
        self.next_state().0
    }
}

impl From<&BTreeMap<String, Attribute>> for State {
    fn from(state: &BTreeMap<String, Attribute>) -> Self {
        State(state.clone(), None, None)
    }
}

#[test]
fn test_dispatch() {
    let state = State::default();
    let old = state.get_hash_code();

    let mut state = state
        .insert("test", 10.0)
        .insert("test", 14.0)
        .next_state();

    let new = state.get_hash_code();
    assert_ne!(old, new); 

    if let Some(v) = state.get("test") {
        assert_eq!(14.0, v.to_owned().into());
    }

    state = state
        .merge(State::default()
        .insert("test", 17.0)
        .insert("test2", 18.0))
        .next_state();

    let newest = state.get_hash_code();
    assert_ne!(new, newest); 

    if let Some(v) = state.get("test") {
        assert_eq!(17.0, v.to_owned().into()); 
    }

    state = state
        .merge(State::default()
            .insert("test", 17.0)
            .insert("test2", 18.0))
        .next_state();

    let latest = state.get_hash_code();
    assert_eq!(newest, latest, "The hash code should not change after an inert merge");
    
    state = state.delete("test2"); 
    assert!(state.get("test2").is_none())
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
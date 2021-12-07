mod attribute;
mod font;
mod gui;
mod node;
mod window;

use imgui::FontSource;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use imnodes::EditorContext;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;
use window::Hardware;
use window::WindowContext;

pub use gui::ControlState;
pub use gui::GUIUpdate;
pub use gui::GUI;

pub use node::expression;
pub use node::expression::*;
pub use node::Display;
pub use node::EditorResource;
pub use node::NodeEditor;
pub use node::NodeExterior;
pub use node::NodeInterior;
pub use node::NodeModule;
pub use node::NodeResource;
pub use node::NodeVisitor;
pub use node::Output;
pub use node::Reducer;
pub use node::Module;
pub use node::Initializer;

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
pub struct State(
    BTreeMap<String, Attribute>,
    Option<Vec<Update>>,
);

#[derive(Clone, Hash)]
pub enum Update {
    Insert(String, Attribute),
    Delete(String),
    Merge(BTreeMap<String, Attribute>),
}

impl Default for State {
    fn default() -> Self {
        Self(BTreeMap::default(), None)
    }
}

impl<'a> NodeVisitor<'a> for State {
    type Parameters = Update;

    fn call(&self, params: Self::Parameters) -> Self {
        let State(mut next,..) = self.clone();
        match params {
            Update::Insert(key, value) => {
                next.insert(key, value.clone());
            }
            Update::Delete(key) => {
                next.remove(&key);
            }
            Update::Merge(map) => {
                for (key, value) in map {
                    next.insert(key, value.to_owned());
                }
            }
        };

        self.merge(next)
    }

    fn evaluate(&self) -> Option<State> {
        match &self {
            State(_, Some(updates)) => {
                let mut next = self.clone();
                updates
                .iter()
                .for_each(|u| next = next.call(u.clone()));

                Some(next)
            }
            _ => None
        }
    }
}

impl State {
    /// `get` returns the latest version of the attribute
    /// `get` will flatten all messages into a state before getting the next value. This has no side effects on the original collection.
    pub fn get(&self, key: &'static str) -> Option<Attribute> {
        if let Some(State(map, ..)) = self.clone().evaluate() {
            match map.get(key) {
                Some(attr) => Some(attr.clone()),
                _ => None
            }
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

    /// `snapshot` returns a clone of state as-is w/o updates
    pub fn snapshot(&self) -> Self {
        State(self.0.clone(), None)
    }

    /// `insert` is a helper method to dispatch an insert update
    pub fn insert<V>(&self, key: &str, value: V) -> Self
    where
        V: Into<Attribute>,
    {
        self.call(Update::Insert(key.to_string(), value.into()))
    }

    /// `merge` is a helper method to dispatch a merge update
    pub fn merge<M>(&self, map: M) -> Self
    where
        M: Into<BTreeMap<String, Attribute>>,
    {
        self.call(Update::Merge(map.into()))
    }

    /// `delete` is a helper method to dispatch a delete update
    pub fn delete(&self, key: &str) -> Self {
        self.call(Update::Delete(key.to_string()))
    }

    /// `map` creates a clone of a subset of parameters from `State`
    pub fn map(&self, parameters: &[&'static str]) -> Self {
        let mut mapped = Self::default();
        parameters
            .iter()
            .map(|p| (p, self.get(p)))
            .filter_map(|p| match p {
                (name, Some(attr)) => Some((name, attr)),
                _ => None,
            })
            .for_each(|(n, a)| {
                mapped = mapped.insert(*n, a);
            });

        mapped
    }

    /// `select` inserts a Select routine at `key`
    pub fn select(&self, key: &str, select: fn(State) -> (u64, Option<Attribute>)) -> Self {
        self.insert(key, Attribute::Functions(Routines::Select(select)))
    }

    /// `reduce` applies the reducer fn and returns the next State
    pub fn reduce<T>(&self, reducer: fn(state: State, t: T) -> Self) -> Self
    where
        T: Reducer + From<State>,
    {
        reducer(self.snapshot(), T::from(self.clone()))
    }

    /// `visit` allows a visitor to initialize from this state
    pub fn visit<'a, T>(&self) -> T::Visitor
    where
        T: NodeInterior<'a>,
    {
        T::accept(self.clone())
    }
}

impl Into<BTreeMap<String, Attribute>> for State {
    fn into(self) -> BTreeMap<String, Attribute> {
        if let Some(next) = self.clone().evaluate() {
            next.0
        } else {
            self.0
        }
    }
}

impl From<&BTreeMap<String, Attribute>> for State {
    fn from(state: &BTreeMap<String, Attribute>) -> Self {
        State(state.clone(), None)
    }
}

#[test]
fn test_dispatch() {
    let state = State::default();
    let old = state.get_hash_code();

    let state = state.insert("test", 10.0).insert("test", 14.0).evaluate().unwrap();

    let new = state.get_hash_code();
    assert_ne!(old, new);

    if let Some(v) = state.get("test") {
        assert_eq!(14.0, v.to_owned().into());
    }

    let evaluated_state = state
        .merge(State::default().insert("test", 17.0).insert("test2", 18.0))
        .evaluate().unwrap();

    let newest = evaluated_state.get_hash_code();
    assert_ne!(new, newest);

    if let Some(v) = state.get("test") {
        assert_eq!(17.0, v.to_owned().into());
    }

    let evaluated_state = state
        .merge(State::default().insert("test", 17.0).insert("test2", 18.0))
        .evaluate().expect("was just created so it should be evaluated");

    let latest = evaluated_state.get_hash_code();
    assert_eq!(
        newest, latest,
        "The hash code should not change after an inert merge"
    );

    let state = state.delete("test2");
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
            }
            Value::TextBuffer(txt) => txt.hash(state),
        };
    }
}

impl Into<Attribute> for Value {
    fn into(self) -> Attribute {
        Attribute::Literal(self)
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
            adapter: Some(adapater),
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

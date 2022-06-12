mod font;
mod gui;
mod window;

use imgui::FontSource;
use imgui::Ui;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use serde::Deserialize;
use serde::Serialize;
use specs::storage::DenseVecStorage;
use specs::Builder;
use specs::Component;
use specs::DispatcherBuilder;
use specs::System;
use specs::World;
use specs::WorldExt;
use std::any::Any;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
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

/// Implement this trait to extend an app's systems
pub trait Extension<'a, 'ui> {
    /// configure_app_world can be implemented by an extension to
    /// register resources and components to the app world
    fn configure_app_world(world: &'a mut World);

    /// configure_app_systems can be implemented by an extension to
    /// register systems that will run on the app world
    fn configure_app_systems(dispatcher: &'a mut DispatcherBuilder);
 
    /// on_ui gets called inside the event loop when the ui is ready
    /// app_world is called here so that systems that aren't already added
    /// have a chance to call run_now, (Note!! this is called on frame processing, use with care)
    fn on_ui(&'a mut self, app_world: &'a World, ui: &'a imgui::Ui<'ui>);
}

#[derive(Clone, Default, Debug, Component, Serialize, Deserialize, Hash)]
#[storage(DenseVecStorage)]
pub struct Attribute {
    pub id: u32,
    pub name: String,
    pub value: Value,
    #[serde(skip)]
    pub transient: Option<(String, Value)>,
}

impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.id, &self.name, &self.value, &self.transient).cmp(&(other.id, &other.name, &other.value, &self.transient))
    }
}

impl Eq for Attribute {
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && 
        self.name == other.name && 
        self.value == other.value && 
        self.transient == other.transient
    }
}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.id, &self.name, &self.value, &self.transient).partial_cmp(&(other.id, &other.name, &other.value, &self.transient))
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}::", self.id)?;
        write!(f, "{}::", self.name)?;
        
        Ok(())
    }
}

impl Into<(String, Value)> for &mut Attribute {
    fn into(self) -> (String, Value) {
        (self.name().to_string(), self.value().clone())
    }
}

impl Attribute {
    pub fn new(id: u32, name: impl AsRef<str>, value: Value) -> Attribute {
        Attribute {
            id,
            name: {
                name.as_ref().to_string()
            },
            value,
            transient: None,
        }
    }

    /// Returns `true` when this attribute is in a `stable` state.
    /// A `stable` state means that there are no pending changes focused on this instance of the `attribute`.
    pub fn is_stable(&self) -> bool {
        self.transient.is_none()
    }

    /// Returns the transient part of this attribute
    pub fn transient(&self) -> Option<&(String, Value)> {
        self.transient.as_ref()
    }

    pub fn take_transient(&mut self) -> Option<(String, Value)> {
        self.transient.take()
    }

    pub fn commit(&mut self) {
        if let Some((name, value)) = &self.transient {
            self.name = name.clone();
            self.value = value.clone();
            self.transient = None;
        }
    }

    pub fn edit_self(&mut self) {
        let init = self.into();
        self.edit(init);
    }

    pub fn edit(&mut self, edit: (String, Value)) {
        self.transient = Some(edit);
    }

    pub fn edit_as(&mut self, edit: Value) {
        if let Some((name, _)) = &self.transient {
            self.transient = Some((name.to_string(), edit));
        } else {
            self.transient = Some((self.name().to_string(), edit));
        }
    }

    pub fn reset_editing(&mut self) {
        if let Some((name, value)) = &mut self.transient {
            *value = self.value.clone();
            *name = self.name.clone();
        }
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
    
    /// write to the current value of this attribute
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    /// read the current id of this attribute
    /// This id is likely the entity owner of this attribute
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl App for Attribute {
    fn name() -> &'static str {
        "Attribute"
    }

    fn show_editor(&mut self, ui: &imgui::Ui) {
        let label = format!("{} {:#4x}", self.name, self.id);

        let editing = if let Some((name, e)) = &mut self.transient {
            let name_label = format!("name of {}", label);
            ui.set_next_item_width(200.0);
            ui.input_text(name_label, name).build();
            if let Value::Reference(r) = self.value.to_ref() {
                ui.text(format!("reference: {:#5x}", r));
            }
            e
        } else {
            &mut self.value
        };

        ui.set_next_item_width(200.0);
        match editing {
            Value::Empty => {
                ui.text("empty");
            }
            Value::Float(float) => {
                ui.input_float(label, float).build();
            }
            Value::Int(int) => {
                ui.input_int(label, int).build();
            }
            Value::Bool(bool) => {
                ui.checkbox(label, bool);
            }
            Value::FloatRange(f1, f2, f3) => {
                let clone = &mut [*f1, *f2, *f3];
                ui.input_float3(label, clone).build();
                *f1 = clone[0];
                *f2 = clone[1];
                *f3 = clone[2];
            }
            Value::IntRange(i1, i2, i3) => {
                let clone = &mut [*i1, *i2, *i3];
                ui.input_int3(label, clone).build();
                *i1 = clone[0];
                *i2 = clone[1];
                *i3 = clone[2];
            }
            Value::TextBuffer(text) => {
                ui.input_text(label, text).build();
            }
            Value::FloatPair(f1, f2) => {
                let clone = &mut [*f1, *f2];
                ui.input_float2(label, clone).build();
                *f1 = clone[0];
                *f2 = clone[1];
            }
            Value::IntPair(i1, i2) => {
                let clone = &mut [*i1, *i2];
                ui.input_int2(label, clone).build();
                *i1 = clone[0];
                *i2 = clone[1];
            }
            Value::BinaryVector(v) => {
                ui.label_text("vector length", format!("{}", v.len()));

                if self.name.starts_with("file::") {
                    if let Some(mut content) = String::from_utf8(v.to_vec()).ok() {
                        ui.input_text_multiline(
                            format!("content of {}", self.name),
                            &mut content,
                            [800.0, 200.0],
                        )
                        .read_only(true)
                        .build();
                    }

                    if ui.button(format!("reload {}", label)) {
                        let name = self.name.to_owned();
                        let filename = &name[6..];
                        match fs::read_to_string(filename) {
                            Ok(string) => {
                                *v = string.as_bytes().to_vec();
                            }
                            Err(err) => {
                                eprintln!("Could not load file '{}', for attribute labeled '{}', entity {}. Error: {}", &filename, label, self.id, err);
                            }
                        }
                    }

                    if ui.button(format!("write to disk {}", label)) {
                        let name = self.name.to_owned();
                        let filename = &name[6..];
                        match fs::write(filename, v) {
                            Ok(_) => {
                                println!("Saved to {}", filename);
                            }
                            Err(err) => {
                                eprintln!("Could not load file '{}', for attribute labeled '{}', entity {}. Error: {}", &filename, label, self.id, err);
                            }
                        }
                    }
                }
            }
            Value::Reference(r) => {
                ui.label_text(label, format!("{:#5x}", r));
            },
            Value::Symbol(symbol) => {
                ui.label_text(label, symbol);
            },
        };
    }
}

impl Attribute {
    /// helper function to show an editor for the internal state of the attribute
    pub fn edit_ui(&mut self, ui: &Ui) {
        if let Some(_) = self.transient {
            self.show_editor(ui);
            if ui.button(format!("save changes [{} {}]", self.name(), self.id)) {
                self.commit();
            }

            ui.same_line();
            if ui.button(format!("reset changes [{} {}]", self.name(), self.id)) {
                self.reset_editing();
            }
        } else {
            self.show_editor(ui);
            if ui.button(format!("edit [{} {}]", self.name(), self.id)) {
                self.transient = Some((self.name.clone(), self.value.clone()));
            }
        }
    }
}

#[derive(Debug, Clone, Component, Serialize, Deserialize, PartialEq, PartialOrd)]
#[storage(DenseVecStorage)]
pub enum Value {
    Empty,
    Bool(bool),
    TextBuffer(String),
    Int(i32),
    IntPair(i32, i32),
    IntRange(i32, i32, i32),
    Float(f32),
    FloatPair(f32, f32),
    FloatRange(f32, f32, f32),
    BinaryVector(Vec<u8>),
    Reference(u64),
    Symbol(String),
}

impl Eq for Value {
    
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            Ordering::Less
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty|
            Value::Symbol(_)|
            Value::Float(_)|
            Value::Int(_)|
            Value::Bool(_)|
            Value::TextBuffer(_)|
            Value::IntPair(_, _)|
            Value::FloatPair(_, _) |
            Value::FloatRange(_, _, _)|
            Value::IntRange(_, _, _) => {
                write!(f, "{:?}", self)?;
            },
            Value::BinaryVector(vec) => {
                write!(f, "{}", base64::encode(vec))?;
            }
            Value::Reference(_) => return write!(f, "{:?}", self),
        }

        let r = self.to_ref();
        write!(f, "::{:?}", r)
    }
}

impl Value {
    pub fn to_ref(&self) -> Value {
        let state = &mut DefaultHasher::default();
        self.hash(state);

        Value::Reference(state.finish())
    }
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
            Value::IntPair(i1, i2) => {
                i1.hash(state);
                i2.hash(state);
            }
            Value::FloatPair(f1, f2) => {
                f1.to_bits().hash(state);
                f2.to_bits().hash(state);
            }
            Value::BinaryVector(v) => {
                v.hash(state);
            }
            Value::Reference(r) => r.hash(state),
            Value::Symbol(r) => r.hash(state),
        };
    }
}

pub fn start_editor<A, F, Ext>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extend: F,
    ext_app: Ext,
) where
    A: App + for<'a> System<'a> + Send,
    F: 'static + Fn(&mut A, &mut World, &mut DispatcherBuilder),
    Ext: 'static + FnMut(&World, &imgui::Ui),
{
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });
    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    // As part of the gui system setup, the gui system will also begin setup of the application system
    let (event_loop, gui) = new_gui_system(title, width, height, app, extend, ext_app);

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
                .insert(gui_entity, GUIUpdate { event })
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

pub fn new_gui_system<A, F, Ext>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extension: F,
    ext_app: Ext,
) -> (winit::event_loop::EventLoop<()>, GUI<A, F, Ext>)
where
    A: App + System<'static>,
    F: FnOnce(&mut A, &mut World, &mut DispatcherBuilder),
    Ext: FnMut(&World, &imgui::Ui),
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
                ext_app,
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

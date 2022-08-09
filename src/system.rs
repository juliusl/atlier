mod font;
mod gui;
mod window;

use imgui::FontSource;
use imgui::Key;
use imgui::MouseButton;
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
use std::str::from_utf8;
use wgpu::util::StagingBelt;
use wgpu::TextureView;
use window::Hardware;
use window::WindowContext;
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event_loop::ControlFlow;

pub use gui::ControlState;
pub use gui::GUIUpdate;
pub use gui::GUI;
pub use winit::event::WindowEvent;

pub use font::cascadia_code;
pub use font::monaco;
pub use font::segoe_ui;

/// The App trait allows for mut/read-only access to component state
pub trait App
where
    Self: Any + Send + Sync,
{
    /// name of this app
    fn name() -> &'static str;

    /// default window_size to use for this app
    fn window_size() -> &'static [f32; 2] {
        &[1920.0, 1080.0]
    }

    /// Show ui that can edit self
    fn edit_ui(&mut self, ui: &imgui::Ui);

    /// Show ui that can display self
    fn display_ui(&self, ui: &imgui::Ui);

    /// Called on start up
    fn on_init(
        &mut self,
        _surface: &wgpu::Surface,
        _config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    /// Enable depth stencil
    fn enable_depth_stencil<'a>(&self) -> bool {
        false
    }

    /// Called when a new frame is ready to be rendered
    fn on_render<'a>(
        &'a mut self,
        _view: &wgpu::TextureView,
        _surface: &wgpu::Surface,
        _config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _rpass: &mut wgpu::RenderPass<'a>,
    ) {
    }
}

/// Implementing this trait allows for extending the event loop runtime
pub trait Extension {
    /// configure_app_world can be implemented by an extension to
    /// register resources and components to the app world
    fn configure_app_world(_world: &mut World) {}

    /// configure_app_systems can be implemented by an extension to
    /// register systems that will run on the app world
    fn configure_app_systems(_dispatcher: &mut DispatcherBuilder) {}

    /// on_ui gets called inside the event loop when the ui is ready
    /// app_world is called here so that systems that aren't already added
    /// have a chance to call run_now, (Note!! this is called on frame processing, use with care)
    fn on_ui(&'_ mut self, _app_world: &World, _ui: &'_ imgui::Ui<'_>) {}

    /// on_window_event gets called on every window event
    fn on_window_event(&'_ mut self, _app_world: &World, _event: &'_ WindowEvent<'_>) {}

    /// on_device_event gets called on every device event
    fn on_device_event(
        &'_ mut self,
        _app_world: &World,
        _device_id: &'_ DeviceId,
        _event: &'_ DeviceEvent,
    ) {
    }

    /// on_run is called on every iteration of run
    /// called before app.run_now(), and before any events are handled by the event_loop
    fn on_run(&'_ mut self, _app_world: &World) {}

    /// on_maintain is called after `.maintain()` is called on the world
    fn on_maintain(&'_ mut self, _app_world: &mut World) {}

    /// on_render_init is called when the renderer pipeline is being setup
    fn on_render_init(
        &'_ mut self,
        _surface: &wgpu::Surface,
        _config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    /// on_render for extensions relies on the encoder/staging_belt
    fn on_render(
        &'_ mut self,
        _view: &wgpu::TextureView,
        _depth_view: Option<&wgpu::TextureView>,
        _surface: &wgpu::Surface,
        _config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        _staging_belt: &mut StagingBelt,
    ) {
    }

    /// standalone sets up a new specs environment with this extension
    fn standalone<'a, 'b>() -> (World, DispatcherBuilder<'a, 'b>) {
        let mut world = World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        Self::configure_app_world(&mut world);
        Self::configure_app_systems(&mut dispatcher_builder);

        (world, dispatcher_builder)
    }
}

/// Returns a tuple of two extensions, which can be used as a single extension
pub fn combine<A, B>(a: A, b: B) -> (A, B)
where
    A: Extension,
    B: Extension,
{
    (a, b)
}

/// Returns a tuple of two default extensions
pub fn combine_default<A, B>() -> (A, B)
where
    A: Extension + Default,
    B: Extension + Default,
{
    (A::default(), B::default())
}

impl<A, B> Extension for (A, B)
where
    A: Extension,
    B: Extension,
{
    fn configure_app_world(world: &mut World) {
        A::configure_app_world(world);
        B::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        A::configure_app_systems(dispatcher);
        B::configure_app_systems(dispatcher);
    }

    fn on_ui(&'_ mut self, app_world: &World, ui: &'_ imgui::Ui<'_>) {
        let (a, b) = self;

        a.on_ui(app_world, ui);
        b.on_ui(app_world, ui);
    }

    fn on_window_event(&'_ mut self, app_world: &World, event: &'_ WindowEvent<'_>) {
        let (a, b) = self;

        a.on_window_event(app_world, event);
        b.on_window_event(app_world, event);
    }

    fn on_device_event(
        &'_ mut self,
        app_world: &World,
        device_id: &'_ DeviceId,
        event: &'_ DeviceEvent,
    ) {
        let (a, b) = self;

        a.on_device_event(app_world, device_id, event);
        b.on_device_event(app_world, device_id, event);
    }

    fn on_run(&'_ mut self, app_world: &World) {
        let (a, b) = self;

        a.on_run(app_world);
        b.on_run(app_world);
    }

    fn on_maintain(&'_ mut self, app_world: &mut World) {
        let (a, b) = self;

        a.on_maintain(app_world);
        b.on_maintain(app_world);
    }

    fn on_render_init(
        &'_ mut self,
        surface: &wgpu::Surface,
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let (a, b) = self;

        a.on_render_init(surface, config, adapter, device, queue);
        b.on_render_init(surface, config, adapter, device, queue);
    }

    fn on_render(
        &'_ mut self,
        view: &wgpu::TextureView,
        depth_view: Option<&wgpu::TextureView>,
        surface: &wgpu::Surface,
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut StagingBelt,
    ) {
        let (a, b) = self;

        a.on_render(
            view,
            depth_view,
            surface,
            config,
            adapter,
            device,
            queue,
            encoder,
            staging_belt,
        );
        b.on_render(
            view,
            depth_view,
            surface,
            config,
            adapter,
            device,
            queue,
            encoder,
            staging_belt,
        );
    }
}

/// An attribute is the main "framing" resource
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
        (self.id, &self.name, &self.value, &self.transient).cmp(&(
            other.id,
            &other.name,
            &other.value,
            &self.transient,
        ))
    }
}

impl Eq for Attribute {}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.value == other.value
            && self.transient == other.transient
    }
}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.id, &self.name, &self.value, &self.transient).partial_cmp(&(
            other.id,
            &other.name,
            &other.value,
            &self.transient,
        ))
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
            name: { name.as_ref().to_string() },
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

    fn display_ui(&self, _: &imgui::Ui) {}

    fn edit_ui(&mut self, ui: &imgui::Ui) {
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
            }
            Value::Symbol(symbol) => {
                ui.label_text(label, symbol);
            }
        };
    }
}

impl Attribute {
    /// helper function to show an editor for the internal state of the attribute
    pub fn edit_attr(&mut self, ui: &Ui) {
        if let Some(_) = self.transient {
            self.edit_ui(ui);
            if ui.button(format!("save changes [{} {}]", self.name(), self.id)) {
                self.commit();
            }

            ui.same_line();
            if ui.button(format!("reset changes [{} {}]", self.name(), self.id)) {
                self.reset_editing();
            }
        } else {
            self.edit_ui(ui);
            if ui.button(format!("edit [{} {}]", self.name(), self.id)) {
                self.transient = Some((self.name.clone(), self.value.clone()));
            }
        }
    }

    /// helper function to show an editor for the internal state of the attribute
    pub fn edit_value(&mut self, with_label: impl AsRef<str>, ui: &Ui) {
        let mut input_label = format!("{} {:#4x}", self.name, self.id);

        if !with_label.as_ref().is_empty() {
            input_label = with_label.as_ref().to_string();
        }

        match self.value_mut() {
            Value::Symbol(_) => {
                if let Some((_, value)) = &mut self.transient {
                    ui.text("(transient)");
                    ui.same_line();
                    value.edit_ui(input_label, ui);
                }
            }
            value => {
                value.edit_ui(input_label, ui);
            }
        };
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

impl Value {
    pub fn edit_ui(&mut self, label: impl AsRef<str>, ui: &imgui::Ui) {
        match self {
            Value::Empty => {
                ui.label_text(label, "empty");
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
                imgui::Slider::new(label, *f2, *f3).build(ui, f1);
            }
            Value::IntRange(i1, i2, i3) => {
                imgui::Slider::new(label, *i2, *i3).build(ui, i1);
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
                ui.label_text(label, format!("{} bytes", v.len()));
                if let Some(text) = from_utf8(v).ok().filter(|s| !s.is_empty()) {
                    let width = text
                        .split_once("\n")
                        .and_then(|(l, ..)| Some(l.len() as f32 * 16.0 + 400.0))
                        .and_then(|w| Some(w.min(1360.0)))
                        .unwrap_or(800.0);

                    if ui.is_item_hovered()
                        && (ui.is_key_down(Key::V) || ui.is_mouse_down(MouseButton::Middle))
                    {
                        ui.tooltip(|| {
                            if !text.is_empty() {
                                ui.text("Preview - Right+Click to pin/expand");
                                ui.input_text_multiline(
                                    "preview-tooltip",
                                    &mut text.to_string(),
                                    [width, 35.0 * 16.0],
                                )
                                .read_only(true)
                                .build();
                            }
                        });
                    }

                    if ui.is_item_hovered()
                        && !ui.is_key_down(Key::V)
                        && !ui.is_mouse_down(MouseButton::Middle)
                    {
                        ui.tooltip_text("Hold+V or Middle+Mouse to peek at content");
                    }

                    ui.popup(&text, || {
                        if !text.is_empty() {
                            ui.text("Preview");
                            ui.input_text_multiline(
                                "preview",
                                &mut text.to_string(),
                                [1360.0, 35.0 * 16.0],
                            )
                            .read_only(true)
                            .build();
                        }
                    });

                    if ui.is_item_clicked_with_button(imgui::MouseButton::Right) {
                        ui.open_popup(&text);
                    }
                }
            }
            Value::Reference(r) => {
                ui.label_text(label, format!("{:#5x}", r));
            }
            Value::Symbol(symbol) => {
                ui.text(symbol);
            }
        };
    }
}

impl Eq for Value {}

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
            Value::Empty
            | Value::Symbol(_)
            | Value::Float(_)
            | Value::Int(_)
            | Value::Bool(_)
            | Value::TextBuffer(_)
            | Value::IntPair(_, _)
            | Value::FloatPair(_, _)
            | Value::FloatRange(_, _, _)
            | Value::IntRange(_, _, _) => {
                write!(f, "{:?}", self)?;
            }
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
    /// Converts to Value::Reference(), 
    /// 
    /// If self is already Value::Reference(), returns self w/o rehashing
    pub fn to_ref(&self) -> Value {
        Value::Reference(match self {
            Value::Reference(r) => *r,
            _ => {
                let state = &mut DefaultHasher::default();
                self.hash(state);
                state.finish()
            }
        })
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

/// Opens a window for some App/Extension
pub fn open_window<A, E>(title: &str, width: f64, height: f64, app: A, extension: E)
where
    A: App + for<'c> System<'c>,
    E: Extension + 'static,
{
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });
    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    // As part of the gui system setup, the gui system will also begin setup of the application system
    let (event_loop, gui) = new_gui_system(title, width, height, app, extension);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new();
    dispatcher.add_thread_local(gui);

    let mut dispatcher = dispatcher.build();
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
        // THREAD LOCAL
        // Dispatch the next event to the gui_entity that is rendering windows
        if let Some(event) = event.to_static() {
            if let Err(err) = w.write_component().insert(gui_entity, GUIUpdate { event }) {
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

/// Creates a graphics/ui pipeline and window, returns the event loop and the pipeline system
fn new_gui_system<A, E>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extension: E,
) -> (winit::event_loop::EventLoop<()>, GUI<A, E>)
where
    A: App + for<'c> System<'c>,
    E: Extension + 'static,
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

            let depth_texture = create_depth_texture(&device, &surface_desc, "depth_texture");

            let renderer_config = RendererConfig {
                texture_format: surface_desc.format,
                depth_format: {
                    if app.enable_depth_stencil() {
                        Some(DEPTH_FORMAT)
                    } else {
                        None
                    }
                },
                ..Default::default()
            };

            let renderer = Renderer::new(setup_imgui, &device, &queue, renderer_config);
            let staging_belt = StagingBelt::new(1024);

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
                depth_texture,
                platform,
                staging_belt,
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

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

fn create_depth_texture<'a>(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    label: &str,
) -> TextureView {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
    };
    let texture = device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    view
}

mod font;
mod gui;
mod window;

use imgui::FontSource;
use imgui_wgpu::Renderer;
use imgui_wgpu::RendererConfig;
use imnodes::IdentifierGenerator;
use imnodes::NodeScope;
use specs::Builder;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;
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

pub type ShowFunc<S> = fn(&imgui::Ui, &S, Option<&mut imnodes::EditorContext>) -> Option<S>;

pub trait App
where
    Self: Clone + Default,
{
    /// title of this app
    fn title() -> &'static str;

    /// default window_size to use for this app
    fn window_size() -> &'static [f64;2] {
        &[1920.0, 1080.0]
    }

    /// start the editor if Self is also the expected State
    fn start_editor(mut initial_state: Option<Self>)
    where
        Self: Clone + Default + 'static,
    {
        let &[width, height] = Self::window_size();
        if let None = initial_state {
            initial_state = Some(Self::default());
        }

        start_editor(
            Self::title(),
            width,
            height,
            initial_state.expect("This should've been the default state"),
            |_, _|{},
            Self::show,
            true,
        );
    }

    /// show's the UI for this app
    fn show(
        ui: &imgui::Ui,
        state: &Self,
        imnode_editor: Option<&mut imnodes::EditorContext>,
    ) -> Option<Self>;

    // show the app's node for this app
    fn show_node(
        _: &imgui::Ui,
        _: &Self,
        _: NodeScope,
        _: &mut IdentifierGenerator,
    ) -> Option<Self> {
        None
    }
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

pub fn default_start_editor_1080p<S>(title: &str, initialize: InitializeWorldFunc, show: ShowFunc<S>)
where
    S: Clone + Default + 'static,
{
    start_editor(title, 1920.0, 1080.0, S::default(), initialize, show, true)
}

type InitializeWorldFunc = fn(&mut World, &mut DispatcherBuilder);

pub fn start_editor<S>(
    title: &str,
    width: f64,
    height: f64,
    initial_state: S,
    initialize: InitializeWorldFunc,
    show: ShowFunc<S>,
    enable_imnodes: bool,
) where
    S: Clone + Default + 'static,
{
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });
    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here

    let (event_loop, gui) =
        new_gui_system::<S>(title, width, height, initial_state, show, enable_imnodes);

    // Create the specs dispatcher
    let mut dispatcher = DispatcherBuilder::new().with_thread_local(gui);
    
    initialize(&mut w, &mut dispatcher);
    
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
        dispatcher.dispatch_seq(&w);

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

pub fn new_gui_system<S>(
    title: &str,
    width: f64,
    height: f64,
    initial_state: S,
    app: ShowFunc<S>,
    enable_imnodes: bool,
) -> (winit::event_loop::EventLoop<()>, GUI<S>)
where
    S: Clone + Default,
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

            // ImNodes needs to be passed directly into the show function
            let imnodes = imnodes::Context::new();
            let editor = imnodes.create_editor();

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
                app,
                last_frame: None,
                last_cursor: None,
                state: initial_state,
                imnodes: {
                    if !enable_imnodes {
                        None
                    } else {
                        Some(imnodes)
                    }
                },
                imnodes_editor: {
                    if !enable_imnodes {
                        None
                    } else {
                        Some(editor)
                    }
                },
            };

            return (event_loop, gui);
        } else {
            panic!("Could not initialize hardware")
        };
    };

    setup()
}

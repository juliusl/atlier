mod attribute;
mod font;
mod gui;
mod value;
mod window;

use imgui::FontSource;
use imgui_wgpu::{Renderer, RendererConfig};
use specs::{Builder, DispatcherBuilder, System, World, WorldExt};
use wgpu::util::StagingBelt;
use wgpu::TextureView;
use window::{Hardware, WindowContext};
use winit::event_loop::ControlFlow;

pub use gui::{ControlState, GUIUpdate, GUI};
pub use winit::event::WindowEvent;

pub use font::{cascadia_code, monaco, segoe_ui};

pub use attribute::Attribute;
pub use value::Value;

pub use crate::app::App;
pub use crate::extension::Extension;
pub use crate::combine::{combine, combine_default};

/// Opens a window for some App/Extension,
///
pub fn open_window<A, E>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extension: E,
    world: Option<World>,
    dispatcher_builder: Option<DispatcherBuilder<'static, 'static>>,
) where
    A: App + for<'c> System<'c>,
    E: Extension + 'static,
{
    // This is the backend world
    let mut w = World::new();
    w.insert(ControlState { control_flow: None });
    // Create the new gui_system,
    // after this point no changes can be made to gui or event_loop
    // This application either starts up, or panics here
    // As part of the gui system setup, the gui system will also begin setup of the application system
    let (event_loop, gui) = new_gui_system(title, width, height, app, extension, world, dispatcher_builder);

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
/// 
fn new_gui_system<A, E>(
    title: &str,
    width: f64,
    height: f64,
    app: A,
    extension: E,
    world: Option<World>,
    dispatcher_builder: Option<DispatcherBuilder<'static, 'static>>,
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
                app_world: world.unwrap_or(World::new()),
                app_dispatcher_builder: dispatcher_builder,
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

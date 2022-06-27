use std::time::Instant;
use specs::prelude::*;
use specs::shred::DynamicSystemData;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

use super::App;
use super::Extension;

pub struct GUI<A, E>
where
    A: App + for<'c> System<'c>,
    E: Extension + 'static,
{
    pub window_title: String,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_desc: wgpu::SurfaceConfiguration,
    pub window: winit::window::Window,
    pub physical_size: winit::dpi::PhysicalSize<u32>,
    pub platform: imgui_winit_support::WinitPlatform,
    pub imgui: imgui::Context,
    pub renderer: imgui_wgpu::Renderer,
    pub hidpi_scale_factor: f64,
    pub font_size: f32,
    pub last_frame: Option<Instant>,
    pub last_cursor: Option<imgui::MouseCursor>,
    pub app: A,
    pub extension: E,
    pub app_world: World,
    pub app_dispatcher: Option<Dispatcher<'static, 'static>>,
}

pub struct GUIUpdate {
    pub event: Event<'static, ()>,
}
impl<'a> Component for GUIUpdate {
    type Storage = HashMapStorage<Self>;
}

pub struct ControlState {
    pub control_flow: Option<ControlFlow>,
}
impl Default for ControlState {
    fn default() -> Self {
        ControlState { control_flow: None }
    }
}

#[derive(SystemData)]
pub struct GUISystemData<'a> {
    control_state: Write<'a, ControlState>,
    update: ReadStorage<'a, GUIUpdate>,
}

impl<'a, A, E> System<'a> for GUI<A, E>
where
    A: App + for<'c> System<'c>,
    E: Extension + 'static,
{
    type SystemData = GUISystemData<'a>;

    fn setup(&mut self, world: &mut World) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), world);

        // Setup app world
        // this is where the extension method is called for the first time and resources/components
        // will get instantiated
        let mut app_world = &mut self.app_world;
        let mut app_dispatcher = DispatcherBuilder::new();

        E::configure_app_world(&mut app_world);
        E::configure_app_systems(&mut app_dispatcher);

        <A::SystemData as DynamicSystemData>::setup(&self.app.accessor(), app_world);
        let mut dispatcher = app_dispatcher.build();
        dispatcher.setup(&mut app_world);
        self.app_dispatcher = Some(dispatcher);
    }

    fn run(&mut self, data: Self::SystemData) {
        // Since we're using a nested world, we need to manually call run_now on the main app
        // since the main app needs the main thread to render it's ui
        // however they systems added in the extension method will continue running independently
        // This is why Sections must declare if they expect state to be modified by extensions.
        // By declaring this, the state must be able to reconcile state that has been updated on a different thread.
        // We can make no inferences at this level of the stack.
        if let Some(app_dispatcher) = &mut self.app_dispatcher {
            app_dispatcher.dispatch(&self.app_world);

            self.extension.on_run(&self.app_world);
            self.extension.on_update(&mut self.app_world);

            // main app will always run last because it needs to be on the main thread
            self.app.run_now(&self.app_world);
            self.app_world.maintain();
        }
        
        let mut control_state = data.control_state;
        for GUIUpdate { event } in data.update.join() {
            control_state.control_flow = Some(ControlFlow::Poll);

            if let Event::WindowEvent {  event: window_event, .. } = event {
                self.extension.on_window_event(&self.app_world, window_event);
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                    ..
                } => {
                    self.hidpi_scale_factor = *scale_factor;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    // Recreate the swap chain with the new size
                    self.surface_desc.width = size.width;
                    self.surface_desc.height = size.height;
                    self.surface.configure(&self.device, &self.surface_desc);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => control_state.control_flow = Some(ControlFlow::Exit),
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawEventsCleared => {
                    let now = Instant::now();
                    if let Some(f) = self.last_frame {
                        self.imgui.io_mut().update_delta_time(now - f);
                    }
                    self.last_frame = Some(now);

                    let frame = match self.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };

                    self.platform
                        .prepare_frame(self.imgui.io_mut(), &self.window)
                        .expect("Failed to prepare frame");

                    let ui = self.imgui.frame();
                    self.extension.on_ui(&self.app_world, &ui);
                    self.app_world.maintain();

                    // This is where we actually render the app's ui
                    // whatever state the app is in at this point is what the ui will see
                    // Repeating this information here from above...
                    // Also, important to note, the ui at this point can make any changes independent of any extensions.
                    // This means if the ui is expecting extensions to make changes, it needs to ensure runtime state knows how to
                    // reconcile this. 
                    self.app.edit_ui(&ui);
                    let _ = &self.app.display_ui(&ui);

                    let mut encoder: wgpu::CommandEncoder = self
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if self.last_cursor != ui.mouse_cursor() {
                        self.last_cursor = ui.mouse_cursor();
                        self.platform.prepare_render(&ui, &self.window);
                    }

                    let view = &frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    self.renderer
                        .render(ui.render(), &self.queue, &self.device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass); // renders to screen on drop, will probaly be changed in wgpu 0.7 or later

                    self.queue.submit(Some(encoder.finish()));
                    frame.present()
                }
                _ => (),
            }

            self.platform
                .handle_event(self.imgui.io_mut(), &self.window, &event);
        }
    }
}

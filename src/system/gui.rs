use std::time::Instant;

use imgui::Ui;
use specs::prelude::*;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

use super::ShowFunc;

pub struct GUI<S>
where
    S: Clone + Default,
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
    pub app: ShowFunc<S>,
    pub state: S,
    pub imnodes: Option<imnodes::Context>,
    pub imnodes_editor: Option<imnodes::EditorContext>,
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

impl<'a, S> System<'a> for GUI<S>
where
    S: Clone + Default,
{
    type SystemData = GUISystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut control_state = data.control_state;

        for GUIUpdate { event } in data.update.join() {
            control_state.control_flow = Some(ControlFlow::Poll);

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

                    let ui: Ui = self.imgui.frame();

                    //ui.show_demo_window(&mut true);

                    let func = self.app;
                    let state = self.state.clone();

                    if let Some(_) = &self.imnodes_editor {
                        if let Some(state) = func(&ui, &state, self.imnodes_editor.as_mut()) {
                            self.state = state;
                        }
                    } else {
                        if let Some(state) = func(&ui, &state, None) {
                            self.state = state;
                        }
                    }

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
                                    g: 0.4,
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

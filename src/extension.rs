use specs::{DispatcherBuilder, World};
use wgpu::util::StagingBelt;
use winit::event::{WindowEvent, DeviceEvent, DeviceId};

/// Implementing this trait gives access to various extension points within the 
/// window's event_loop.
/// 
pub trait Extension {
    /// configure_app_world can be implemented by an extension to
    /// register resources and components to the app world
    fn configure_app_world(_world: &mut World) {}

    /// configure_app_systems can be implemented by an extension to
    /// register systems that will run on the app world
    fn configure_app_systems(_dispatcher: &mut DispatcherBuilder) {}

    /// Configures imgui context, called on gui setup
    /// 
    fn configure_imgui_context(_context: &mut imgui::Context) {}

    /// on_ui gets called inside the event loop when the ui is ready
    /// app_world is called here so that systems that aren't already added
    /// have a chance to call run_now, (Note!! this is called on frame processing, use with care)
    fn on_ui(&'_ mut self, _app_world: &World, _ui: &'_ imgui::Ui) {}

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
}
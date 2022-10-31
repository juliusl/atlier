use std::any::Any;

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

    /// Edit imgui context
    fn edit_style(&self, _context: &mut imgui::Style) {}

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
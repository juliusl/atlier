use specs::{World, DispatcherBuilder};
use wgpu::util::StagingBelt;
use winit::event::{DeviceId, DeviceEvent, WindowEvent};

use crate::system::Extension;

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
use egui_wgpu::wgpu;
use egui_winit::winit;
use simworld_core::math::size::Size;
use simworld_core::renderer::{RenderStage, Renderer};
use simworld_core::world::view::ViewState;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            renderer: Renderer::new(device),
        }
    }

    pub fn prepare(&mut self, view_state: &ViewState, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.renderer.prepare(view_state, device, queue);
    }

    pub fn render(&mut self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Game Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        self.renderer.render(&mut rpass);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer
            .resize_screen(Size::new(size.width as f32, size.height as f32));
    }
}

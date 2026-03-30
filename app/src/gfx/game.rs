use egui_wgpu::wgpu;
use egui_winit::winit;
use simworld_core::math::point::Point;
use simworld_core::math::size::Size;
use simworld_core::renderer::Renderer;
use simworld_core::visuals::state::VisualState;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> anyhow::Result<Self> {
        Ok(Self {
            renderer: Renderer::new(device, surface_format)?,
        })
    }

    pub fn prepare(&mut self, visuals: &VisualState, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.renderer.prepare(visuals, device, queue);
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

    pub fn pan_camera(&mut self, dx: f64, dy: f64) {
        self.renderer.camera_pan((dx, dy));
    }

    pub fn zoom_camera(&mut self, scroll_delta: f64, mouse_x: f64, mouse_y: f64) {
        self.renderer
            .camera_zoom(scroll_delta, Point::new(mouse_x, mouse_y));
    }
}

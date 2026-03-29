use crate::math::point::Point;
use crate::math::size::Size;
use crate::world::view::ViewState;
use wgpu::{Device, Queue, RenderPass};

mod camera;

pub struct Renderer {
    camera: camera::CameraStage,
}

impl Renderer {
    pub fn new(device: &Device) -> Self {
        Self {
            camera: camera::CameraStage::new(device),
        }
    }

    pub fn resize_screen(&mut self, size: Size<f32>) {
        self.camera.resize(size);
    }

    pub fn camera_pan(&mut self, screen_delta: (f64, f64)) {
        self.camera.pan(screen_delta);
    }

    pub fn camera_zoom(&mut self, scroll_delta: f64, mouse_pos: Point<f64>) {
        self.camera.zoom(scroll_delta, mouse_pos);
    }
}

impl RenderStage for Renderer {
    fn prepare(&mut self, view: &ViewState, device: &Device, queue: &Queue) {
        self.camera.prepare(view, device, queue);
    }

    fn render<'rp>(&'rp self, pass: &mut RenderPass<'rp>) {
        self.camera.render(pass);
    }
}

pub trait RenderStage {
    fn prepare(&mut self, view: &ViewState, device: &wgpu::Device, queue: &wgpu::Queue);

    fn render<'rp>(&'rp self, pass: &mut wgpu::RenderPass<'rp>);
}

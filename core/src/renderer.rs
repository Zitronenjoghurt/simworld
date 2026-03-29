use crate::error::CoreResult;
use crate::math::point::Point;
use crate::math::size::Size;
use crate::visuals::palette::Palette;
use crate::visuals::state::VisualState;
use wgpu::{Device, Queue, RenderPass};

mod atlas;
mod camera;
mod palette;

pub struct Renderer {
    camera: camera::CameraStage,
    palette: palette::PaletteStage,
}

impl Renderer {
    pub fn new(device: &Device) -> CoreResult<Self> {
        Ok(Self {
            camera: camera::CameraStage::new(device),
            palette: palette::PaletteStage::new(Palette::default(), device),
        })
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
    fn prepare(&mut self, visuals: &VisualState, device: &Device, queue: &Queue) {
        self.camera.prepare(visuals, device, queue);
    }

    fn render<'rp>(&'rp self, pass: &mut RenderPass<'rp>) {
        self.camera.render(pass);
    }
}

pub trait RenderStage {
    fn prepare(&mut self, view: &VisualState, device: &wgpu::Device, queue: &wgpu::Queue);

    fn render<'rp>(&'rp self, pass: &mut wgpu::RenderPass<'rp>);
}

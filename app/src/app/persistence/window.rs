use egui_winit::winit;
use egui_winit::winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct WindowState {
    pub position: Option<(i32, i32)>,
    pub size: (u32, u32),
    pub maximized: bool,
}

impl WindowState {
    pub fn from_window(window: &winit::window::Window) -> Self {
        Self {
            position: window.outer_position().ok().map(|p| (p.x, p.y)),
            size: (window.inner_size().width, window.inner_size().height),
            maximized: window.is_maximized(),
        }
    }

    pub fn apply(&self, attrs: winit::window::WindowAttributes) -> winit::window::WindowAttributes {
        let mut attrs = attrs.with_inner_size(PhysicalSize::new(self.size.0, self.size.1));
        if let Some(pos) = self.position {
            attrs = attrs.with_position(PhysicalPosition::new(pos.0, pos.1));
        }
        if self.maximized {
            attrs = attrs.with_maximized(true);
        }
        attrs
    }
}

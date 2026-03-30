use egui_winit::winit::dpi::PhysicalPosition;
use egui_winit::winit::event::{ElementState, MouseButton};

#[derive(Default)]
pub struct AppUiControls {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub middle_mouse_button: bool,
}

impl AppUiControls {
    pub fn mouse_pos(&mut self, pos: PhysicalPosition<f64>) {
        self.mouse_x = pos.x;
        self.mouse_y = pos.y;
    }

    pub fn mouse_button(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Middle {
            self.middle_mouse_button = state == ElementState::Pressed;
        }
    }
}

use egui_winit::winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod gfx;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

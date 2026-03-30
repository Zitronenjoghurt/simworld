use crate::gfx::Gfx;
use egui_winit::winit::application::ApplicationHandler;
use egui_winit::winit::event::WindowEvent;
use egui_winit::winit::event_loop::ActiveEventLoop;
use egui_winit::winit::window::{Window, WindowId};
use simworld_core::sim::Sim;
use simworld_core::world::World;
use std::sync::Arc;

pub mod controls;

pub struct App<'a> {
    pub gfx: Option<Gfx<'a>>,
    pub sim: Sim,
    pub controls: controls::AppUiControls,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gfx: None,
            sim: Sim::new(World::new(200, 200)),
            controls: controls::AppUiControls::default(),
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title("Simworld");
        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.gfx = Some(Gfx::new(window).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(gfx) = &mut self.gfx else { return };
        let _consumed = gfx.on_window_event(&event, &mut self.controls);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => gfx.render().unwrap(),
            WindowEvent::Resized(size) => gfx.resize(size),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let sim_state = self.sim.latest_state();

        if let Some(gfx) = &mut self.gfx {
            gfx.prepare(&sim_state.visuals);
            gfx.request_redraw();
        }
    }
}

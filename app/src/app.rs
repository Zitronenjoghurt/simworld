use crate::gfx::Gfx;
use crate::ui::{AppUi, UiContext};
use egui_winit::winit::application::ApplicationHandler;
use egui_winit::winit::event::WindowEvent;
use egui_winit::winit::event_loop::ActiveEventLoop;
use egui_winit::winit::window::{Window, WindowId};
use simworld_core::sim::config::SimConfig;
use simworld_core::sim::Sim;
use simworld_core::world::World;
use std::sync::Arc;

pub struct App<'a> {
    pub gfx: Option<Gfx<'a>>,
    pub sim: Sim,
    pub ui: AppUi,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gfx: None,
            sim: Sim::new(SimConfig::default(), World::new(500, 500)),
            ui: AppUi::default(),
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
        let _consumed = gfx.on_window_event(&event, &mut self.ui.controls);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let sim_state = self.sim.latest_state();
                gfx.prepare(&sim_state.visuals);

                let mut ui_ctx = UiContext {
                    gfx_cpu: gfx.performance().cpu,
                    gfx_gpu: gfx.performance().gpu,
                    sim_performance: &sim_state.performance,
                };

                gfx.render(|ui| self.ui.show(ui, &mut ui_ctx)).unwrap();
            }
            WindowEvent::Resized(size) => gfx.resize(size),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(gfx) = &mut self.gfx {
            gfx.request_redraw();
        }
    }
}

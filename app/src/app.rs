use crate::app::persistence::window::WindowState;
use crate::app::persistence::AppStateWrite;
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

mod directories;
mod persistence;

pub struct App<'a> {
    pub gfx: Option<Gfx<'a>>,
    pub sim: Sim,
    pub ui: AppUi,
    last_save: std::time::Instant,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gfx: None,
            sim: Sim::new(SimConfig::default(), World::new(500, 500)),
            ui: AppUi::default(),
            last_save: std::time::Instant::now(),
        }
    }

    pub fn save(&self) {
        let Some(gfx) = &self.gfx else { return };

        gfx.get_egui_context().memory(|mem| {
            let window = WindowState::from_window(gfx.window());
            let state_write = AppStateWrite {
                app: &self.ui,
                egui: mem,
                window,
            };
            state_write.save().unwrap();
        });
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let state = persistence::AppStateRead::load().unwrap();

        let mut attrs = Window::default_attributes().with_title("Simworld");
        if let Some(state) = &state {
            attrs = state.window.apply(attrs);
        }

        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let gfx = Gfx::new(window).unwrap();

        if let Some(state) = state {
            gfx.get_egui_context().memory_mut(|m| *m = state.egui);
            self.ui = state.app;
        }

        self.gfx = Some(gfx);
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
            WindowEvent::CloseRequested => {
                self.save();
                event_loop.exit();
            }
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

        if self.last_save.elapsed().as_secs() > 10 {
            self.save();
            self.last_save = std::time::Instant::now();
        }
    }
}

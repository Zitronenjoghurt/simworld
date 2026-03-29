use crate::gfx::egui::Egui;
use crate::gfx::game::Game;
use crate::gfx::performance::GfxPerformance;
use crate::gfx::wgpu::Wgpu;
use egui_winit::winit;
use egui_winit::winit::dpi::PhysicalSize;
use egui_winit::winit::window::Window;
use simworld_core::visuals::state::VisualState;
use std::sync::Arc;

mod egui;
mod game;
mod performance;
mod wgpu;

pub struct Gfx<'a> {
    wgpu: Wgpu<'a>,
    egui: Egui,
    game: Game,
    performance: GfxPerformance,
}

impl Gfx<'_> {
    pub fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let wgpu = Wgpu::setup(window.clone())?;
        let egui = Egui::setup(&wgpu);
        let game = Game::new(wgpu.device())?;
        let performance = GfxPerformance::new(wgpu.device())?;

        let gfx = Gfx {
            wgpu,
            egui,
            game,
            performance,
        };

        Ok(gfx)
    }

    pub fn prepare(&mut self, visuals: &VisualState) {
        self.game
            .prepare(visuals, self.wgpu.device(), self.wgpu.queue());
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.performance.start_frame();

        let Some(surface_texture) = self.wgpu.surface_texture()? else {
            return Ok(());
        };

        let view = surface_texture.texture.create_view(&Default::default());
        let mut encoder = self.wgpu.command_encoder();

        {
            let scope = self
                .performance
                .get_scope("Full Render Frame", &mut encoder);
            self.game.render(&view, scope.recorder);
            self.egui.render(&self.wgpu, &view, scope.recorder);
        }

        self.performance.resolve_queries(&mut encoder);

        self.wgpu.queue().submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        self.performance
            .end_frame(self.wgpu.queue().get_timestamp_period())?;
        Ok(())
    }

    pub fn request_redraw(&self) {
        self.wgpu.window().request_redraw();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.wgpu.resize(size);
        self.game.resize(size);
    }

    pub fn on_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        if self.egui.on_window_event(self.wgpu.window(), event) {
            return true;
        };

        false
    }

    pub fn set_egui_scale_factor(&mut self, scale_factor: f32) {
        self.egui.set_scale_factor(scale_factor);
    }
}

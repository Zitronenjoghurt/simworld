use crate::gfx::egui::Egui;
use crate::gfx::game::Game;
use crate::gfx::performance::GfxPerformance;
use crate::gfx::wgpu::Wgpu;
use crate::ui::controls::AppUiControls;
use ::egui::{Context, Ui};
use egui_winit::winit;
use egui_winit::winit::dpi::PhysicalSize;
use egui_winit::winit::event::MouseScrollDelta;
use egui_winit::winit::window::Window;
use simworld_core::visuals::state::VisualState;
use std::sync::Arc;

mod egui;
mod game;
pub mod performance;
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
        let game = Game::new(wgpu.device(), wgpu.surface_format())?;
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

    pub fn render(&mut self, build_ui: impl FnMut(&mut Ui)) -> anyhow::Result<()> {
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
            self.egui
                .render(&self.wgpu, &view, scope.recorder, build_ui);
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
        self.game.resize(self.wgpu.device(), size);
    }

    pub fn on_window_event(
        &mut self,
        event: &winit::event::WindowEvent,
        controls: &mut AppUiControls,
    ) -> bool {
        self.egui.on_window_event(self.wgpu.window(), event);

        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                if controls.middle_mouse_button {
                    let dx = position.x - controls.mouse_x;
                    let dy = position.y - controls.mouse_y;
                    self.game.pan_camera(dx, dy);
                }
                controls.mouse_pos(*position);
            }
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let scroll_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => (pos.y / 50.0) as f32,
                };

                if scroll_y != 0.0 {
                    self.game
                        .zoom_camera(scroll_y as f64, controls.mouse_x, controls.mouse_y);
                }
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                controls.mouse_button(*state, *button);
            }
            _ => {}
        }

        false
    }

    pub fn set_egui_scale_factor(&mut self, scale_factor: f32) {
        self.egui.set_scale_factor(scale_factor);
    }

    pub fn get_egui_context(&self) -> &Context {
        self.egui.get_egui_ctx()
    }

    pub fn performance(&self) -> &GfxPerformance {
        &self.performance
    }

    pub fn window(&self) -> &Window {
        self.wgpu.window()
    }
}

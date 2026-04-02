use egui::{Panel, Ui};
use simworld_core::math::ema::EmaTimer;
use simworld_core::sim::performance::SimPerformance;

pub mod controls;
mod windows;

pub struct UiContext<'a> {
    pub gfx_cpu: EmaTimer,
    pub gfx_gpu: EmaTimer,
    pub sim_performance: &'a SimPerformance,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AppUi {
    #[serde(skip, default)]
    pub controls: controls::AppUiControls,
}

impl AppUi {
    pub fn show(&mut self, ui: &mut Ui, ctx: &mut UiContext) {
        Panel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tick:");
                ui.label(ctx.sim_performance.tick.to_string());
                ui.separator();
                ui.label("Update:");
                ui.label(ctx.sim_performance.update.to_string());
                ui.separator();
                ui.label("Render CPU:");
                ui.label(ctx.gfx_cpu.to_string());
                ui.separator();
                ui.label("Render GPU:");
                ui.label(ctx.gfx_gpu.display_average_secs());
            });
        });
    }
}

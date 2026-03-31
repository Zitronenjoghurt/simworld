use egui_wgpu::wgpu::CommandEncoder;
use simworld_core::math::ema::EmaTimer;
use std::time::Duration;
use wgpu_profiler::{GpuProfiler, GpuProfilerSettings, Scope};

pub struct GfxPerformance {
    pub cpu: EmaTimer,
    pub gpu: EmaTimer,
    profiler: GpuProfiler,
}

impl GfxPerformance {
    pub fn new(device: &egui_wgpu::wgpu::Device) -> anyhow::Result<Self> {
        let profiler = GpuProfiler::new(device, GpuProfilerSettings::default())?;
        Ok(Self {
            cpu: EmaTimer::new(),
            gpu: EmaTimer::new(),
            profiler,
        })
    }

    pub fn get_scope<'a>(
        &'a self,
        label: &str,
        encoder: &'a mut CommandEncoder,
    ) -> Scope<'a, CommandEncoder> {
        self.profiler.scope(label, encoder)
    }

    pub fn resolve_queries(&mut self, encoder: &mut CommandEncoder) {
        self.profiler.resolve_queries(encoder);
    }

    pub fn start_frame(&mut self) {
        self.cpu.start();
    }

    pub fn end_frame(&mut self, timestamp_period: f32) -> anyhow::Result<()> {
        self.profiler.end_frame()?;

        if let Some(results) = self.profiler.process_finished_frame(timestamp_period)
            && let Some(root_query) = results.first()
            && let Some(time_range) = &root_query.time
            && let Ok(duration) = Duration::try_from_secs_f64(time_range.end - time_range.start)
        {
            self.gpu.update(duration, 100);
        }

        self.cpu.stop(1800);

        Ok(())
    }
}

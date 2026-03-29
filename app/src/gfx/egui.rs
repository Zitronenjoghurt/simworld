use crate::gfx::wgpu::Wgpu;
use egui::{ClippedPrimitive, Panel};
use egui_wgpu::{wgpu, RendererOptions};
use egui_winit::winit;

pub struct Egui {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
    screen: egui_wgpu::ScreenDescriptor,
    textures: egui::TexturesDelta,
}

impl Egui {
    pub fn setup(wgpu: &Wgpu) -> Self {
        let state = state(wgpu);
        let renderer = renderer(wgpu);
        let screen = screen(wgpu);

        Egui {
            state,
            renderer,
            screen,
            textures: Default::default(),
        }
    }

    fn prepare_ui(&mut self, wgpu: &Wgpu) -> Vec<ClippedPrimitive> {
        let size = wgpu.window().inner_size();
        self.screen.size_in_pixels = [size.width, size.height];
        self.screen.pixels_per_point = self.state.egui_ctx().pixels_per_point();

        let input = self.state.take_egui_input(wgpu.window());
        let output = self.state.egui_ctx().run_ui(input, |ctx| {
            Panel::top("top_pannel").show_inside(ctx, |ui| {
                ui.heading("Welcome to SimWorld!");
            });
        });

        self.textures.append(output.textures_delta);
        self.state
            .handle_platform_output(wgpu.window(), output.platform_output);
        self.state
            .egui_ctx()
            .tessellate(output.shapes, self.screen.pixels_per_point)
    }

    pub fn render(
        &mut self,
        wgpu: &Wgpu,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let paint_jobs = self.prepare_ui(wgpu);

        for (id, delta) in &self.textures.set {
            self.renderer
                .update_texture(wgpu.device(), wgpu.queue(), *id, delta);
        }

        self.renderer.update_buffers(
            wgpu.device(),
            wgpu.queue(),
            encoder,
            &paint_jobs,
            &self.screen,
        );

        let mut rpass = encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            })
            .forget_lifetime();

        self.renderer.render(&mut rpass, &paint_jobs, &self.screen);

        for id in &std::mem::take(&mut self.textures).free {
            self.renderer.free_texture(id);
        }
    }

    pub fn on_window_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) -> bool {
        self.state.on_window_event(window, event).consumed
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.screen.pixels_per_point
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.state.egui_ctx().set_pixels_per_point(scale_factor);
    }
}

fn context() -> egui::Context {
    let egui_ctx = egui::Context::default();

    //let mut fonts = FontDefinitions::default();
    //fonts.font_data.insert(
    //    "phosphor".into(),
    //    Arc::from(egui::FontData::from_static(
    //        egui_phosphor::Variant::Regular.font_bytes(),
    //    )),
    //);
    //if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
    //    font_keys.insert(1, "phosphor".into());
    //}
    //egui_ctx.set_fonts(fonts);

    egui_ctx
}

fn state(wgpu: &Wgpu) -> egui_winit::State {
    let max_tex = wgpu.device().limits().max_texture_dimension_2d as usize;
    egui_winit::State::new(
        context(),
        egui::ViewportId::ROOT,
        wgpu.window(),
        None,
        None,
        Some(max_tex),
    )
}

fn renderer(wgpu: &Wgpu) -> egui_wgpu::Renderer {
    egui_wgpu::Renderer::new(
        wgpu.device(),
        wgpu.surface_format(),
        RendererOptions::default(),
    )
}

fn screen(wgpu: &Wgpu) -> egui_wgpu::ScreenDescriptor {
    egui_wgpu::ScreenDescriptor {
        size_in_pixels: [
            wgpu.window().inner_size().width,
            wgpu.window().inner_size().height,
        ],
        pixels_per_point: wgpu.window().scale_factor() as f32,
    }
}

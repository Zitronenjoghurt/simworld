use crate::error::CoreResult;
use crate::math::point::Point;
use crate::math::size::Size;
use crate::visuals::palette::Palette;
use crate::visuals::sprite_atlas::SpriteAtlas;
use crate::visuals::state::VisualState;

mod atlas;
mod camera;
mod palette;
mod sprite;

pub struct Renderer {
    camera: camera::CameraStage,
    palette: palette::PaletteStage,
    atlas: atlas::AtlasStage,
    sprites: sprite::SpriteStage,
    depth_view: wgpu::TextureView,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> CoreResult<Self> {
        let palette = Palette::default();
        let atlas = SpriteAtlas::load(&palette)?;

        let camera_stage = camera::CameraStage::new(device);
        let palette_stage = palette::PaletteStage::new(palette, device);
        let atlas_stage = atlas::AtlasStage::new(atlas, device);
        let sprite_stage = sprite::SpriteStage::new(
            device,
            surface_format,
            &camera_stage,
            &palette_stage,
            &atlas_stage,
        );

        Ok(Self {
            camera: camera_stage,
            palette: palette_stage,
            atlas: atlas_stage,
            sprites: sprite_stage,
            depth_view: depth_view(device, 1, 1),
        })
    }

    pub fn prepare(&mut self, visuals: &VisualState, _device: &wgpu::Device, queue: &wgpu::Queue) {
        self.camera.update(queue);
        self.palette.update(queue);
        self.atlas.update(queue);
        self.sprites.prepare(visuals, &self.atlas, queue);
    }

    pub fn render<'rp>(&'rp self, pass: &mut wgpu::RenderPass<'rp>) {
        self.sprites
            .render(pass, &self.camera, &self.palette, &self.atlas);
    }

    pub fn resize_screen(&mut self, device: &wgpu::Device, size: Size<f32>) {
        self.camera.resize(size);
        self.depth_view = depth_view(device, size.width as u32, size.height as u32);
    }

    pub fn camera_pan(&mut self, screen_delta: (f64, f64)) {
        self.camera.pan(screen_delta);
    }

    pub fn camera_zoom(&mut self, scroll_delta: f64, mouse_pos: Point<f64>) {
        self.camera.zoom(scroll_delta, mouse_pos);
    }

    pub fn depth_view(&self) -> &wgpu::TextureView {
        &self.depth_view
    }
}

fn depth_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default())
}

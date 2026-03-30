use crate::visuals::sprite::SpriteId;
use crate::visuals::sprite_atlas::SpriteAtlas;
use bytemuck::{Pod, Zeroable};

pub struct AtlasStage {
    atlas: SpriteAtlas,
    dirty: bool,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    info_buffer: wgpu::Buffer,
    texture: wgpu::Texture,
}

impl AtlasStage {
    pub fn new(atlas: SpriteAtlas, device: &wgpu::Device) -> Self {
        let texture = texture(&atlas, device);
        let view = texture.create_view(&Default::default());
        let info_buffer = info_buffer(device);
        let (bind_group, bind_group_layout) = bind_group(device, &view, &info_buffer);

        Self {
            atlas,
            dirty: true,
            bind_group,
            bind_group_layout,
            info_buffer,
            texture,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.dirty {
            write_texture(&self.atlas, &self.texture, queue);
            let info = AtlasInfo {
                sprites_per_row: (self.atlas.sprite_count() as u32).isqrt().max(1),
                sprite_size: 8,
                _padding: [0; 2],
            };
            queue.write_buffer(&self.info_buffer, 0, bytemuck::bytes_of(&info));
            self.dirty = false;
        }
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn atlas_index(&self, id: SpriteId) -> Option<usize> {
        self.atlas.atlas_index(id)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct AtlasInfo {
    pub sprites_per_row: u32,
    pub sprite_size: u32,
    pub _padding: [u32; 2],
}

fn texture(atlas: &SpriteAtlas, device: &wgpu::Device) -> wgpu::Texture {
    let count = atlas.sprite_count() as u32;
    let sprites_per_row = count.isqrt();
    let rows = count.div_ceil(sprites_per_row);

    let width = sprites_per_row * 8;
    let height = rows.max(1) * 8;

    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Sprite Atlas"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Uint,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

fn write_texture(atlas: &SpriteAtlas, texture: &wgpu::Texture, queue: &wgpu::Queue) {
    let sprites_per_row = atlas.sprite_count().isqrt();
    let width = texture.width();
    let height = texture.height();

    let mut pixels = vec![0u8; width as usize * height as usize];

    for (i, sprite) in atlas.sprites().enumerate() {
        let grid_x = ((i % sprites_per_row) * 8) as u32;
        let grid_y = ((i / sprites_per_row) * 8) as u32;
        for y in 0..8u32 {
            for x in 0..8u32 {
                let dst = ((grid_y + y) * width + grid_x + x) as usize;
                pixels[dst] = u8::from(sprite.raw_pixel((y * 8 + x) as usize));
            }
        }
    }

    queue.write_texture(
        texture.as_image_copy(),
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(width),
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
}

fn info_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("atlas_info_buffer"),
        size: size_of::<AtlasInfo>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn bind_group(
    device: &wgpu::Device,
    view: &wgpu::TextureView,
    info_buffer: &wgpu::Buffer,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("atlas_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Uint,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("atlas_bind_group"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: info_buffer.as_entire_binding(),
            },
        ],
    });

    (group, layout)
}

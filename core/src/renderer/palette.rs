use crate::visuals::palette::Palette;

pub struct PaletteStage {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    palette: Palette,
    palette_dirty: bool,
    texture: wgpu::Texture,
}

impl PaletteStage {
    pub fn new(palette: Palette, device: &wgpu::Device) -> Self {
        let texture = texture(device);
        let view = texture.create_view(&Default::default());
        let (bind_group, bind_group_layout) = bind_group(device, &view);

        Self {
            bind_group,
            bind_group_layout,
            palette,
            palette_dirty: true,
            texture,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.palette_dirty {
            write_texture(&self.palette, &self.texture, queue);
            self.palette_dirty = false;
        }
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

fn texture(device: &wgpu::Device) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Palette"),
        size: wgpu::Extent3d {
            width: 32,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

fn write_texture(palette: &Palette, texture: &wgpu::Texture, queue: &wgpu::Queue) {
    let mut bytes = [0u8; 32 * 4];
    for (i, color) in palette.colors().iter().enumerate() {
        bytes[i * 4] = color.r();
        bytes[i * 4 + 1] = color.g();
        bytes[i * 4 + 2] = color.b();
        bytes[i * 4 + 3] = color.a();
    }

    queue.write_texture(
        texture.as_image_copy(),
        &bytes,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(32 * 4),
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width: 32,
            height: 1,
            depth_or_array_layers: 1,
        },
    );
}

fn bind_group(
    device: &wgpu::Device,
    view: &wgpu::TextureView,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("palette_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        }],
    });

    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("palette_bind_group"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(view),
        }],
    });

    (group, layout)
}

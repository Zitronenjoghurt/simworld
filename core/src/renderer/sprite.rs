use crate::renderer::atlas::AtlasStage;
use crate::renderer::camera::CameraStage;
use crate::renderer::palette::PaletteStage;
use crate::visuals::state::VisualState;
use bytemuck::{Pod, Zeroable};

const MAX_INSTANCES: u32 = 1_048_576;

pub struct SpriteStage {
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
}

impl SpriteStage {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        camera: &CameraStage,
        palette: &PaletteStage,
        atlas: &AtlasStage,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/sprite.wgsl"));

        let layout = pipeline_layout(device, camera, palette, atlas);
        let pipeline = pipeline(device, &layout, &shader, surface_format);
        let instance_buffer = instance_buffer(device);

        Self {
            pipeline,
            instance_buffer,
            instance_count: 0,
        }
    }

    pub fn prepare(&mut self, visuals: &VisualState, atlas: &AtlasStage, queue: &wgpu::Queue) {
        let instances: Vec<SpriteInstance> = visuals
            .sprites()
            .filter_map(|pos| {
                let idx = atlas.atlas_index(pos.sprite_id)?;
                Some(SpriteInstance {
                    world_pos: [pos.x, pos.y, pos.z],
                    atlas_index: idx as u32,
                })
            })
            .collect();

        self.instance_count = (instances.len() as u32).min(MAX_INSTANCES);
        if self.instance_count > 0 {
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&instances[..self.instance_count as usize]),
            );
        }
    }

    pub fn render<'rp>(
        &'rp self,
        pass: &mut wgpu::RenderPass<'rp>,
        camera: &'rp CameraStage,
        palette: &'rp PaletteStage,
        atlas: &'rp AtlasStage,
    ) {
        if self.instance_count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera.bind_group(), &[]);
        pass.set_bind_group(1, palette.bind_group(), &[]);
        pass.set_bind_group(2, atlas.bind_group(), &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        pass.draw(0..6, 0..self.instance_count);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SpriteInstance {
    pub world_pos: [f32; 3],
    pub atlas_index: u32,
}

impl SpriteInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Uint32,
    ];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

fn pipeline_layout(
    device: &wgpu::Device,
    camera: &CameraStage,
    palette: &PaletteStage,
    atlas: &AtlasStage,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sprite_pipeline_layout"),
        bind_group_layouts: &[
            Some(camera.bind_group_layout()),
            Some(palette.bind_group_layout()),
            Some(atlas.bind_group_layout()),
        ],
        immediate_size: 0,
    })
}

fn pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    surface_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("sprite_pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[SpriteInstance::layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        cache: None,
        multiview_mask: None,
    })
}

fn instance_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Sprite Instance Buffer"),
        size: (MAX_INSTANCES as u64) * (size_of::<SpriteInstance>() as u64),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

use crate::visuals::sprite_atlas::SpriteAtlas;

pub struct AtlasStage {
    atlas: SpriteAtlas,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    texture: wgpu::Texture,
}

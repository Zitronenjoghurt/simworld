struct CameraUniform {
    view_proj: mat4x4<f32>,
    screen_size: vec2<f32>,
    _padding: vec2<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

@group(1) @binding(0) var palette_tex: texture_2d<f32>;

@group(2) @binding(0) var atlas_tex: texture_2d<u32>;

struct AtlasInfo {
    sprites_per_row: u32,
    sprite_size: u32,
    _padding: vec2<u32>,
};
@group(2) @binding(1) var<uniform> atlas_info: AtlasInfo;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_uv: vec2<f32>,
    @location(1) @interpolate(flat) atlas_index: u32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vid: u32,
    @location(0) world_pos: vec3<f32>,
    @location(1) atlas_index: u32,
) -> VertexOutput {
    var corner: vec2<f32>;
    switch vid {
        case 0u: { corner = vec2(0.0, 0.0); }
        case 1u: { corner = vec2(1.0, 0.0); }
        case 2u: { corner = vec2(0.0, 1.0); }
        case 3u: { corner = vec2(1.0, 0.0); }
        case 4u: { corner = vec2(1.0, 1.0); }
        case 5u: { corner = vec2(0.0, 1.0); }
        default: { corner = vec2(0.0, 0.0); }
    }

    let sz = f32(atlas_info.sprite_size);
    let world = vec4<f32>(
        world_pos.x + corner.x * sz,
        world_pos.y + corner.y * sz,
        world_pos.z,
        1.0,
    );

    var out: VertexOutput;
    out.clip_pos = camera.view_proj * world;
    out.local_uv = corner;
    out.atlas_index = atlas_index;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let spr = atlas_info.sprites_per_row;
    let sz  = atlas_info.sprite_size;

    let grid_x = in.atlas_index % spr;
    let grid_y = in.atlas_index / spr;

    let local_x = clamp(u32(in.local_uv.x * f32(sz)), 0u, sz - 1u);
    let local_y = clamp(u32(in.local_uv.y * f32(sz)), 0u, sz - 1u);

    let tex_coord = vec2<u32>(grid_x * sz + local_x, grid_y * sz + local_y);
    let palette_idx = textureLoad(atlas_tex, tex_coord, 0).r;

    if (palette_idx & 0x20u) != 0u {
        discard;
    }

    let color_idx = palette_idx & 0x1Fu;
    let color = textureLoad(palette_tex, vec2<u32>(color_idx, 0u), 0);
    return color;
}
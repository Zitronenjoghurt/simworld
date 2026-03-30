use crate::math::point::Point;
use crate::math::rect::Rect;
use crate::math::size::Size;
use bytemuck::{Pod, Zeroable};

pub struct CameraStage {
    camera: Camera,
    dirty: bool,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraStage {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = buffer(device);
        let (bind_group, bind_group_layout) = bind_group(device, &buffer);
        Self {
            camera: Camera::default(),
            dirty: true,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.dirty {
            let uniform = CameraUniform::from(&self.camera);
            queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
            self.dirty = false;
        }
    }

    pub fn resize(&mut self, size: Size<f32>) {
        self.camera.resize(size);
        self.dirty = true;
    }

    pub fn pan(&mut self, screen_delta: (f64, f64)) {
        self.camera.pan(screen_delta);
        self.dirty = true;
    }

    pub fn zoom(&mut self, scroll_delta: f64, mouse_pos: Point<f64>) {
        self.camera.zoom(scroll_delta, mouse_pos);
        self.dirty = true;
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

pub struct Camera {
    screen_size: Size<f32>,
    center: Point<f32>,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            screen_size: Size::new(1.0, 1.0),
            center: Point::default(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn resize(&mut self, size: Size<f32>) {
        self.screen_size = size;
    }

    pub fn pan(&mut self, screen_delta: (f64, f64)) {
        self.center.x -= screen_delta.0 as f32 / self.zoom;
        self.center.y -= screen_delta.1 as f32 / self.zoom;
    }

    pub fn zoom(&mut self, scroll_delta: f64, mouse_pos: Point<f64>) {
        let old_zoom = self.zoom;

        let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 1.0 / 1.1 };
        let new_zoom = old_zoom * zoom_factor;

        let screen_w = self.screen_size.width;
        let screen_h = self.screen_size.height;

        let offset_x = mouse_pos.x as f32 - (screen_w / 2.0);
        let offset_y = mouse_pos.y as f32 - (screen_h / 2.0);
        let adjustment_factor = (1.0 / old_zoom) - (1.0 / new_zoom);

        self.center.x += offset_x * adjustment_factor;
        self.center.y += offset_y * adjustment_factor;
        self.zoom = new_zoom;
    }

    pub fn visible_bounds(&self) -> Rect<f32> {
        let world_width = self.screen_size.width / self.zoom;
        let world_height = self.screen_size.height / self.zoom;
        let half_width = world_width / 2.0;
        let half_height = world_height / 2.0;

        Rect::new(
            Point::new(self.center.x - half_width, self.center.y - half_height),
            Point::new(self.center.x + half_width, self.center.y + half_height),
        )
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub screen_size: [f32; 2],
    pub _padding: [f32; 2],
}

impl CameraUniform {
    pub fn from_screen_and_view(screen_size: Size<f32>, view_rect: Rect<f32>) -> Self {
        let left = view_rect.min.x;
        let right = view_rect.max.x;
        let bottom = view_rect.max.y;
        let top = view_rect.min.y;

        let w_inv = 1.0 / (right - left);
        let h_inv = 1.0 / (top - bottom);

        let view_proj = [
            [2.0 * w_inv, 0.0, 0.0, 0.0],
            [0.0, 2.0 * h_inv, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-(right + left) * w_inv, -(top + bottom) * h_inv, 0.0, 1.0],
        ];

        Self {
            view_proj,
            screen_size: [screen_size.width, screen_size.height],
            _padding: [0.0, 0.0],
        }
    }
}

impl From<&Camera> for CameraUniform {
    fn from(view: &Camera) -> Self {
        Self::from_screen_and_view(view.screen_size, view.visible_bounds())
    }
}

fn buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Camera Buffer"),
        size: size_of::<CameraUniform>() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn bind_group(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("camera_bind_group_layout"),
    });

    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });

    (group, layout)
}

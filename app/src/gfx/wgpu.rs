use anyhow::Context;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::CurrentSurfaceTexture;
use egui_winit::winit;
use egui_winit::winit::window::Window;
use std::sync::Arc;

pub struct Wgpu<'a> {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
}

impl Wgpu<'_> {
    pub fn setup(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = instance();
        let surface = instance.create_surface(window.clone())?;
        let adapter = adapter(&instance, &surface)?;
        let (device, queue) = device_queue(&adapter)?;
        let surface_config = configure_surface(&surface, size, &adapter, &device)?;

        let wgpu = Wgpu {
            window,
            device,
            queue,
            surface,
            surface_config,
        };
        Ok(wgpu)
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.surface_config.width = size.width;
            self.surface_config.height = size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}

// Access helpers
impl Wgpu<'_> {
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&'_ self) -> &'_ wgpu::Surface<'_> {
        &self.surface
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    pub fn surface_texture(&self) -> anyhow::Result<Option<wgpu::SurfaceTexture>> {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture)
            | CurrentSurfaceTexture::Suboptimal(texture) => Ok(Some(texture)),
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => Ok(None),
            CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.surface_config);
                match self.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture)
                    | CurrentSurfaceTexture::Suboptimal(texture) => Ok(Some(texture)),
                    other => Err(anyhow::anyhow!(
                        "Surface texture failed after reconfigure: {:?}",
                        other
                    )),
                }
            }
            _ => Err(anyhow::anyhow!(
                "Surface texture failed: {:?}",
                self.surface.get_current_texture()
            )),
        }
    }

    pub fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&Default::default())
    }
}

fn instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    })
}

pub fn adapter(wgpu: &wgpu::Instance, surface: &wgpu::Surface) -> anyhow::Result<wgpu::Adapter> {
    pollster::block_on(wgpu.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }))
    .context("Failed to find a GPU adapter")
}

pub fn device_queue(adapter: &wgpu::Adapter) -> anyhow::Result<(wgpu::Device, wgpu::Queue)> {
    pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Device"),
        required_features: wgpu::Features::TIMESTAMP_QUERY
            | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS,
        ..Default::default()
    }))
    .context("Failed to create device")
}

fn configure_surface(
    surface: &wgpu::Surface,
    size: winit::dpi::PhysicalSize<u32>,
    adapter: &wgpu::Adapter,
    device: &wgpu::Device,
) -> anyhow::Result<wgpu::SurfaceConfiguration> {
    let capabilities = surface.get_capabilities(adapter);
    let format = capabilities
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .context("Failed to find sRGB format")?;

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width.max(1),
        height: size.height.max(1),
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(device, &config);
    Ok(config)
}

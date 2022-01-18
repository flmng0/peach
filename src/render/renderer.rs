use thiserror::Error;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::graphics::{BufferData, Graphics};
use crate::tess;
use crate::types::{GpuScalar, RawVertex, Scalar, Transform, Vector};

#[derive(Error, Debug)]
pub enum RendererInitError {
    #[error("Failed to request GPU Adapter")]
    RequestAdapter,
    #[error("Failed to request GPU Device")]
    RequestDevice,
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Failed to retrieve current surface texture: {0}")]
    SurfaceTexture(wgpu::SurfaceError),
    #[error("Error when constructing vertex buffers: {0:?}")]
    BufferConstruct(tess::TessellationError),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
    normalize: [GpuScalar; 16],
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    fn generate(width: u32, height: u32) -> Self {
        let width = width as Scalar;
        let height = height as Scalar;

        let transform =
            Transform::scale(2.0 / width, -2.0 / height).then_translate(Vector::new(-1.0, 1.0));

        Self {
            normalize: transform.cast::<GpuScalar>().to_3d().to_array(),
        }
    }
}

pub(crate) struct Renderer {
    surface: wgpu::Surface,

    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::SurfaceConfiguration,

    uniforms_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, RendererInitError> {
        let physical = window.inner_size();

        let PhysicalSize { width, height } = physical;

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .ok_or(RendererInitError::RequestAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .or(Err(RendererInitError::RequestDevice))?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&adapter)
                .unwrap_or_else(|| wgpu::TextureFormat::Bgra8UnormSrgb),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let uniforms = Uniforms::generate(width, height);
        let uniforms_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;
        let uniforms_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniforms bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(uniforms_size),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniforms bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buf.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            multiview: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: core::mem::size_of::<RawVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,
                        1 => Float32x4,
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[config.format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            uniforms_buf,
            bind_group,
            pipeline,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;

            let uniforms = Uniforms::generate(size.width, size.height);
            self.queue
                .write_buffer(&self.uniforms_buf, 0, &bytemuck::bytes_of(&uniforms));

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, gfx: Graphics) -> Result<(), RenderError> {
        let clear_color = gfx.clear_color;

        let BufferData { vertices, indices } = gfx
            .construct_buffer_data()
            .or_else(|err| Err(RenderError::BufferConstruct(err)))?;
        let indice_count = indices.len();

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Peach::Vertex"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Peach::Index"),
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        let output = self
            .surface
            .get_current_texture()
            .or_else(|err| Err(RenderError::SurfaceTexture(err)))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if let Some(color) = clear_color {
                            wgpu::LoadOp::Clear(wgpu::Color {
                                r: color.r as f64,
                                g: color.g as f64,
                                b: color.b as f64,
                                a: color.a as f64,
                            })
                        }
                        else {
                            wgpu::LoadOp::Load
                        },
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));

            rpass.draw_indexed(0..indice_count as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

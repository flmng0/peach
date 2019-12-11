use crate::{tess, Drawable, Size, Transform, Vertex, VertexBuffers};
use winit::{dpi::PhysicalSize, window::Window};

/// TODO: Document `RendererError`
#[derive(Debug)]
pub enum RendererError {
    AdapterRequestError,
    ShadercInitError,
    ShaderCompileError(shaderc::Error),
    ShaderReadError(std::io::Error),
}

/// TODO: Document `Renderer`
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    uniforms: Uniforms,
    uniforms_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    surface: wgpu::Surface,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline: wgpu::RenderPipeline,

    vertex_buffers: VertexBuffers,
}

impl Renderer {
    pub(crate) fn new(window: &Window) -> Result<Renderer, RendererError> {
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::all(),
        })
        .ok_or(RendererError::AdapterRequestError)?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let physical = window.inner_size().to_physical(window.hidpi_factor());

        let uniforms = Uniforms::new(physical);

        let uniforms_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: Uniforms::SIZE as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniforms_buf,
                    range: 0..Uniforms::SIZE as wgpu::BufferAddress,
                },
            }],
        });

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: physical.width.round() as _,
            height: physical.height.round() as _,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let surface = wgpu::Surface::create(window);
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let (vs_module, fs_module) = Self::init_shaders(&device)?;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: Vertex::SIZE as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        offset: 0,
                        format: wgpu::VertexFormat::Float2,
                        shader_location: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: 8,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 1,
                    },
                ],
            }],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok(Renderer {
            device,
            queue,

            uniforms,
            uniforms_buf,
            bind_group,

            surface,
            swap_chain_desc,
            swap_chain,

            pipeline,

            vertex_buffers: tess::VertexBuffers::new(),
        })
    }

    /// Draw any drawable object using this renderer.
    ///
    /// TODO: Example snippet for `Renderer::draw`
    pub fn draw<D>(&mut self, drawable: D)
    where
        D: Drawable,
    {
        drawable.draw(&mut self.vertex_buffers);
    }

    pub(crate) fn resize(&mut self, physical: PhysicalSize) {
        let Self {
            device,
            uniforms,
            surface,
            swap_chain_desc,
            swap_chain,
            ..
        } = self;

        uniforms.size.width = physical.width as _;
        uniforms.size.height = physical.height as _;

        swap_chain_desc.width = physical.width.round() as _;
        swap_chain_desc.height = physical.height.round() as _;

        *swap_chain = device.create_swap_chain(surface, swap_chain_desc);
    }

    pub(crate) fn finish(&mut self) {
        let Self {
            device,
            queue,
            uniforms,
            uniforms_buf,
            bind_group,
            swap_chain,
            pipeline,
            vertex_buffers: VertexBuffers { vertices, indices },
            ..
        } = self;

        let vbo = device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);

        let ibo = device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let transfer_buf = device
            .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[*uniforms]);

        encoder.copy_buffer_to_buffer(
            &transfer_buf,
            0,
            uniforms_buf,
            0,
            Uniforms::SIZE as wgpu::BufferAddress,
        );

        let frame = swap_chain.get_next_texture();
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::WHITE,
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.set_index_buffer(&ibo, 0);
            rpass.set_vertex_buffers(0, &[(&vbo, 0)]);

            rpass.draw_indexed(0..indices.len() as _, 0, 0..1);
        }

        queue.submit(&[encoder.finish()]);

        vertices.clear();
        indices.clear();
    }

    fn create_shader(
        device: &wgpu::Device,
        compiler: &mut shaderc::Compiler,
        source: &str,
        kind: shaderc::ShaderKind,
        name: &str,
    ) -> Result<wgpu::ShaderModule, RendererError> {
        let binary = compiler
            .compile_into_spirv(source, kind, name, "main", None)
            .or_else(|err| Err(RendererError::ShaderCompileError(err)))?;

        let cursor = std::io::Cursor::new(&binary.as_binary_u8()[..]);

        let shader =
            wgpu::read_spirv(cursor).or_else(|err| Err(RendererError::ShaderReadError(err)))?;

        Ok(device.create_shader_module(&shader))
    }

    fn init_shaders(
        device: &wgpu::Device,
    ) -> Result<(wgpu::ShaderModule, wgpu::ShaderModule), RendererError> {
        let mut compiler = shaderc::Compiler::new().ok_or(RendererError::ShadercInitError)?;

        let vs_source = include_str!("shaders/shader.vert");
        let vs_module = Self::create_shader(
            device,
            &mut compiler,
            vs_source,
            shaderc::ShaderKind::Vertex,
            "shaders/shader.vert",
        )?;

        let fs_source = include_str!("shaders/shader.frag");
        let fs_module = Self::create_shader(
            device,
            &mut compiler,
            fs_source,
            shaderc::ShaderKind::Fragment,
            "shaders/shader.frag",
        )?;

        Ok((vs_module, fs_module))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
    pub(crate) size: Size,
    padding: [f32; 2],
    pub(crate) transform: Transform,
}

impl Uniforms {
    const SIZE: usize = std::mem::size_of::<Uniforms>();

    pub(crate) fn new(physical: PhysicalSize) -> Uniforms {
        Uniforms {
            size: Size::new(physical.width as _, physical.height as _),
            padding: [0.0; 2],
            transform: Transform::identity(),
        }
    }
}

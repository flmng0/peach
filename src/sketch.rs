use crate::{
    color::Color,
    draw::{DrawState, Drawing},
    vertex::{Vertex, VertexBuffer},
    Size, Transform,
};

use winit::window::Window;

pub const MAX_STATE_STACK: usize = 64;

#[derive(Debug)]
struct GpuState {
    device: wgpu::Device,
    queue: wgpu::Queue,

    transforms_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    surface: wgpu::Surface,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline: wgpu::RenderPipeline,
}

#[derive(Debug)]
pub struct Sketch {
    gpu_state: GpuState,

    size: Size,
    clear_color: Option<Color>,

    transform: Transform,
    state_stack: Vec<DrawState>,

    fill_buffer: VertexBuffer,
    stroke_buffer: VertexBuffer,
}

impl Sketch {
    pub(crate) fn new(window: &Window) -> Self {
        // Reguest a device adapter, used to retrieve a physical
        // device.
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::all(),
        })
        .unwrap();

        // Device and queue. A device is used to create new objects
        // on the GPU, and a queue is used to submit commands, such
        // as rendering to the GPU.
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        // Size used later for swap chain creation.
        let size = {
            let physical = window.inner_size().to_physical(window.hidpi_factor());

            Size::new(physical.width as _, physical.height as _)
        };

        // Size of the `Transform` buffer in bytes.
        let size_of_transforms = std::mem::size_of::<Transform>();

        let transforms_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: size_of_transforms as _,
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
                    buffer: &transforms_buf,
                    range: 0..size_of_transforms as _,
                },
            }],
        });

        // Surface for drawing commands to write to.
        let surface = wgpu::Surface::create(window);
        // The swap chain descriptor is stored separately, so that
        // it can be re-used when the window is resized.
        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width as _,
            height: size.height as _,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        // Shader modules.
        let [vs_module, fs_module] = init_shaders(&device);

        // Pipline descriptors.
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        // Pipeline descriptor.
        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            // Vertex shader.
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            // Fragment shader.
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            // Rasterization state: vertex direction which defines front face, culling, depth
            // biases, etc.
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            // Which type of primitive is used for tessellation, and constructing primitives on the
            // GPU.
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8Unorm,
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
            // Vertex buffer layout, for Peach, this is Position taking up two floats (4 * 2 = 8
            // bytes), and then Color taking up 4 floats (4 * 4 = 16), then the state stack index
            // taking up 1 u32.
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<Vertex>() as _,
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
            // MSAA sample count.
            sample_count: 1,
            // Bit mask defining which pixels are sampled in MSAA, `!0` forces all bits to be used.
            sample_mask: !0,
            // Whether the GPU stores the original alpha value of a pixel during MSAA, and uses it
            // to calculate later values.
            alpha_to_coverage_enabled: false,
        };
        let pipeline = device.create_render_pipeline(&pipeline_descriptor);

        let state_stack = Vec::with_capacity(MAX_STATE_STACK);
        let transform = view_transform(size);

        Sketch {
            gpu_state: GpuState {
                device,
                queue,

                transforms_buf,
                bind_group,

                surface,
                swap_chain_desc,
                swap_chain,

                pipeline,
            },
            size,
            clear_color: Some(Color::WHITE),

            transform,
            state_stack,

            fill_buffer: VertexBuffer::new(),
            stroke_buffer: VertexBuffer::new(),
        }
    }

    pub(crate) fn resize(&mut self, new_size: Size) {
        let Self {
            gpu_state:
                GpuState {
                    device,
                    surface,
                    swap_chain_desc,
                    swap_chain,
                    ..
                },
            size,
            transform,
            ..
        } = self;

        *size = new_size;

        swap_chain_desc.width = size.width as _;
        swap_chain_desc.height = size.height as _;

        *swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        *transform = view_transform(*size);
    }

    pub(crate) fn finish(&mut self) {
        {
            let Self {
                gpu_state:
                    GpuState {
                        device,
                        queue,
                        transforms_buf,
                        bind_group,
                        swap_chain,
                        pipeline,
                        ..
                    },
                transform,
                fill_buffer:
                    VertexBuffer {
                        vertices: fill_vertices,
                        indices: fill_indices,
                    },
                stroke_buffer:
                    VertexBuffer {
                        vertices: stroke_vertices,
                        indices: stroke_indices,
                    },
                ..
            } = self;

            // Command recorder.
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            // Copy uniforms to GPU.
            let transfer_buf = device.create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC);

            transfer_buf.data[0] = transform.to_column_major_array();

            let size_of_transforms = std::mem::size_of::<Transform>();

            encoder.copy_buffer_to_buffer(
                &transfer_buf.finish(),
                0,
                &transforms_buf,
                0,
                size_of_transforms as _,
            );

            let frame = swap_chain.get_next_texture();

            // Fill render pass.
            {
                let vbo = device
                    .create_buffer_mapped(fill_vertices.len(), wgpu::BufferUsage::VERTEX)
                    .fill_from_slice(&fill_vertices);
                let ibo = device
                    .create_buffer_mapped(fill_indices.len(), wgpu::BufferUsage::INDEX)
                    .fill_from_slice(&fill_indices);

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::TRANSPARENT,
                    }],
                    depth_stencil_attachment: None,
                });

                rpass.set_pipeline(&pipeline);
                rpass.set_bind_group(0, &bind_group, &[]);
                rpass.set_index_buffer(&ibo, 0);
                rpass.set_vertex_buffers(0, &[(&vbo, 0)]);

                rpass.draw_indexed(0..fill_indices.len() as _, 0, 0..1);
            }

            // Stroke render pass.
            {
                let vbo = device
                    .create_buffer_mapped(stroke_vertices.len(), wgpu::BufferUsage::VERTEX)
                    .fill_from_slice(&stroke_vertices);
                let ibo = device
                    .create_buffer_mapped(stroke_indices.len(), wgpu::BufferUsage::INDEX)
                    .fill_from_slice(&stroke_indices);

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::TRANSPARENT,
                    }],
                    depth_stencil_attachment: None,
                });

                rpass.set_pipeline(&pipeline);
                rpass.set_bind_group(0, &bind_group, &[]);
                rpass.set_index_buffer(&ibo, 0);
                rpass.set_vertex_buffers(0, &[(&vbo, 0)]);

                rpass.draw_indexed(0..stroke_indices.len() as _, 0, 0..1);
            }

            queue.submit(&[encoder.finish()]);
        }

        self.clear();
    }

    pub fn push(&mut self) {
        let draw_state = match self.state_stack.last() {
            Some(draw_state) => draw_state.clone(),
            None => DrawState::default(),
        };
        self.state_stack.push(draw_state);
    }

    pub fn pop(&mut self) {
        self.state_stack.pop();
    }
}

impl Drawing for Sketch {
    fn draw_state(&mut self) -> &mut DrawState {
        self.state_stack.last_mut().unwrap()
    }

    fn fill_buffer(&mut self) -> &mut VertexBuffer {
        &mut self.fill_buffer
    }

    fn stroke_buffer(&mut self) -> &mut VertexBuffer {
        &mut self.stroke_buffer
    }

    fn size(&self) -> Size {
        self.size
    }
}

/// Generate a shader module from source code, given a
/// compiler.
fn create_shader(
    device: &wgpu::Device,
    compiler: &mut shaderc::Compiler,
    source: &str,
    name: &str,
    kind: shaderc::ShaderKind,
) -> wgpu::ShaderModule {
    let binary = compiler
        .compile_into_spirv(source, kind, name, "main", None)
        .unwrap();

    let cursor = std::io::Cursor::new(&binary.as_binary_u8()[..]);
    let shader = wgpu::read_spirv(cursor).unwrap();

    device.create_shader_module(&shader)
}

/// Initialize all required shader modules.
///
/// Outputs `[vertex shader module, fragment shader
/// module]`.
fn init_shaders(device: &wgpu::Device) -> [wgpu::ShaderModule; 2] {
    use shaderc::{Compiler, ShaderKind};

    let mut compiler = Compiler::new().unwrap();

    let vs_module = create_shader(
        device,
        &mut compiler,
        include_str!("shader/shader.vert"),
        "shader/shader.vert",
        ShaderKind::Vertex,
    );

    let fs_module = create_shader(
        device,
        &mut compiler,
        include_str!("shader/shader.frag"),
        "shader/shader.frag",
        ShaderKind::Fragment,
    );

    [vs_module, fs_module]
}

fn view_transform(size: Size) -> Transform {
    let mut transform = Transform::identity();

    transform.m11 = 1.0 / size.width * 2.0;
    transform.m22 = 1.0 / size.height * 2.0;

    transform.m14 = -1.0;
    transform.m24 = -1.0;

    transform
}

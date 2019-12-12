use crate::{
    color::Color,
    vertex::{Vertex, VertexBuffer},
    Point, Rect, Size, Transform, Vector,
};

use shaderc::{Compiler, ShaderKind};
use tess::{BuffersBuilder, FillOptions, LineCap, StrokeOptions};
use winit::window::Window;

#[derive(Debug)]
/// Used in [`Sketch::anchor`][0]; describes the anchor
/// point where all geometries are drawn from.
///
/// Default Value: `Anchor::TopLeft`.
///
/// # Applies To:
/// - `rect`
///
/// [0]: struct.Sketch.html#method.anchor
pub enum Anchor {
    /// Place the top-left corner of geometries under the given position.
    TopLeft,
    /// Place geometries in the center of the given position.
    Center,
    /// Offset geometries by the given vector pixels.
    Offset(Vector),
    /// Offset by the given percentage.
    Percent(Vector),
}

impl Default for Anchor {
    fn default() -> Anchor {
        Anchor::TopLeft
    }
}

#[derive(Debug)]
struct GpuState {
    device: wgpu::Device,
    queue: wgpu::Queue,

    uniforms_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    surface: wgpu::Surface,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline: wgpu::RenderPipeline,
}

#[derive(Debug, Default)]
struct DrawState {
    clear_color: Option<Color>,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,

    anchor: Anchor,

    fill_options: FillOptions,
    stroke_options: StrokeOptions,
}

#[derive(Debug)]
pub struct Sketch {
    gpu_state: GpuState,
    draw_state: DrawState,

    uniforms: Uniforms,
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

        // Size of the `Uniforms` buffer in bytes.
        let size_of_uniforms = std::mem::size_of::<Uniforms>();

        let uniforms_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: size_of_uniforms as _,
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
                    range: 0..size_of_uniforms as _,
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
            primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
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
            // bytes), and then Color taking up 4.
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

        let fill_options = FillOptions::default();

        let mut stroke_options = StrokeOptions::default();
        stroke_options.start_cap = LineCap::Round;
        stroke_options.end_cap = LineCap::Round;

        Sketch {
            gpu_state: GpuState {
                device,
                queue,

                uniforms_buf,
                bind_group,

                surface,
                swap_chain_desc,
                swap_chain,

                pipeline,
            },
            draw_state: DrawState {
                clear_color: Some(Color::WHITE),
                fill_options,
                stroke_options,
                ..DrawState::default()
            },
            uniforms: Uniforms::new(size),
            fill_buffer: VertexBuffer::new(),
            stroke_buffer: VertexBuffer::new(),
        }
    }

    pub(crate) fn resize(&mut self, size: Size) {
        let GpuState {
            device,
            surface,
            swap_chain_desc,
            swap_chain,
            ..
        } = &mut self.gpu_state;

        swap_chain_desc.width = size.width as _;
        swap_chain_desc.height = size.height as _;

        *swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
    }

    pub(crate) fn finish(&mut self) {
        let Self {
            gpu_state:
                GpuState {
                    device,
                    queue,
                    uniforms_buf,
                    bind_group,
                    swap_chain,
                    pipeline,
                    ..
                },
            draw_state: DrawState { clear_color, .. },
            uniforms,
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
        let transfer_uniform_buf = device
            .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[*uniforms]);

        encoder.copy_buffer_to_buffer(
            &transfer_uniform_buf,
            0,
            &uniforms_buf,
            0,
            std::mem::size_of::<Uniforms>() as _,
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
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: match clear_color {
                        Some(color) => (*color).into(),
                        None => wgpu::Color::TRANSPARENT,
                    },
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

        fill_vertices.clear();
        fill_indices.clear();

        stroke_vertices.clear();
        stroke_indices.clear();
    }

    pub fn no_clear(&mut self) {
        self.draw_state.clear_color = None;
    }

    pub fn clear<C: Into<Color>>(&mut self, color: C) {
        self.draw_state.clear_color = Some(color.into());
    }

    pub fn no_fill(&mut self) {
        self.draw_state.fill_color = None;
    }

    pub fn fill<C: Into<Color>>(&mut self, color: C) {
        self.draw_state.fill_color = Some(color.into());
    }

    pub fn no_stroke(&mut self) {
        self.draw_state.stroke_color = None;
    }

    pub fn stroke<C: Into<Color>>(&mut self, color: C) {
        self.draw_state.stroke_color = Some(color.into());
    }

    pub fn stroke_width(&mut self, width: f32) {
        self.draw_state.stroke_options.line_width = width;
    }

    pub fn translate(&mut self, translation: Vector) {
        let transform = &mut self.uniforms.transform;

        *transform = transform.pre_translate(translation.to_3d());
    }

    // TODO Remove this
    pub fn temp_clear(&mut self) {
        self.uniforms.transform = Transform::identity();
    }

    pub fn rotate(&mut self, degrees: f32) {
        let transform = &mut self.uniforms.transform;

        let angle = euclid::Angle::degrees(degrees);

        *transform = transform.pre_rotate(0.0, 0.0, 1.0, angle);
    }

    pub fn anchor(&mut self, anchor: Anchor) {
        self.draw_state.anchor = anchor;
    }

    fn apply_anchor(&self, pos: Point, size: Size) -> Point {
        match self.draw_state.anchor {
            Anchor::TopLeft => pos,
            Anchor::Center => pos - Vector::from(size / 2.0),
            Anchor::Offset(offset) => pos - offset,
            Anchor::Percent(offset) => {
                pos - Vector::new(offset.x * size.width, offset.y * size.height)
            }
        }
    }

    pub fn rect<P, S>(&mut self, pos: P, size: S)
    where
        P: Into<Point>,
        S: Into<Size>,
    {
        let size = size.into();
        let pos = self.apply_anchor(pos.into(), size);
        let rect = Rect::new(pos, size);

        if let Some(fill_color) = self.draw_state.fill_color {
            tess::basic_shapes::fill_rectangle(
                &rect,
                &self.draw_state.fill_options,
                &mut BuffersBuilder::new(&mut self.fill_buffer, fill_color),
            )
            .unwrap();
        }

        if let Some(stroke_color) = self.draw_state.stroke_color {
            tess::basic_shapes::stroke_rectangle(
                &rect,
                &self.draw_state.stroke_options,
                &mut BuffersBuilder::new(&mut self.stroke_buffer, stroke_color),
            )
            .unwrap();
        }
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
    let mut compiler = Compiler::new().unwrap();

    let vs_source = include_str!("shader/shader.vert");
    let vs_module = create_shader(
        device,
        &mut compiler,
        vs_source,
        "shader/shader.vert",
        ShaderKind::Vertex,
    );

    let fs_source = include_str!("shader/shader.frag");
    let fs_module = create_shader(
        device,
        &mut compiler,
        fs_source,
        "shader/shader.frag",
        ShaderKind::Fragment,
    );

    [vs_module, fs_module]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    pub(crate) transform: Transform,
    pub(crate) size: Size,
    padding: [f32; 2],
}

impl Uniforms {
    pub(crate) fn new(size: Size) -> Uniforms {
        Uniforms {
            transform: Transform::identity(),
            size,
            padding: [0.0; 2],
        }
    }
}

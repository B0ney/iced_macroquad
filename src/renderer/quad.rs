use bytemuck::{Pod, Zeroable};
use iced_core::{Rectangle, Transformation};
use iced_graphics::{color, Viewport};

use crate::mq::{self, *};

const MAX_QUADS: usize = 1000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;

/// The properties of a quad.
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Quad {
    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The border color of the [`Quad`], in __linear RGB__.
    pub border_color: [f32; 4],

    /// The border radii of the [`Quad`].
    pub border_radius: [f32; 4],

    /// The border width of the [`Quad`].
    pub border_width: f32,
    // /// The shadow color of the [`Quad`].
    // pub shadow_color: [f32; 4],

    // /// The shadow offset of the [`Quad`].
    // pub shadow_offset: [f32; 2],

    // /// The shadow blur radius of the [`Quad`].
    // pub shadow_blur_radius: f32,
}

impl Quad {
    // TODO: index buffer
    fn bindings(ctx: &mut Context) -> mq::Bindings {
        let quad_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::empty::<Quad>(MAX_QUADS),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Dynamic,
            BufferSource::empty::<i32>(MAX_VERTICES * 12),
        );

        Bindings {
            vertex_buffers: vec![quad_vertex_buffer], //todo
            index_buffer: index_buffer,
            images: vec![],
        }
    }

    fn pipeline(ctx: &mut Context) -> mq::Pipeline {
        if ctx.info().backend == mq::Backend::Metal {
            unimplemented!("Metal is not supported.");
        }

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: include_str!("shader/quad.vert"),
                    fragment: include_str!("shader/quad.frag"),
                },
                ShaderMeta {
                    images: vec![],
                    uniforms: Uniforms::uniforms(),
                },
            )
            .unwrap();

        let attributes = &[
            VertexAttribute::new("i_Pos", VertexFormat::Float2),
            VertexAttribute::new("i_Size", VertexFormat::Float2),
            VertexAttribute::new("i_BorderColor", VertexFormat::Float4),
            VertexAttribute::new("i_BorderRadius", VertexFormat::Float4),
            VertexAttribute::new("i_BorderWidth", VertexFormat::Float1),
            // VertexAttribute::new("i_shadow_color", VertexFormat::Float4),
            // VertexAttribute::new("i_shadow_offset", VertexFormat::Float2),
            // VertexAttribute::new("i_shadow_blur_radius", VertexFormat::Float1),
        ];

        ctx.new_pipeline(
            &[mq::BufferLayout {
                step_func: VertexStep::PerInstance,
                ..Default::default()
            }],
            attributes,
            shader,
            mq::PipelineParams::default(),
        )
    }
}

pub struct Pipeline {
    pub pipeline: mq::Pipeline,
    pub bindings: mq::Bindings,
}

impl Pipeline {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            pipeline: Quad::pipeline(ctx),
            bindings: Quad::bindings(ctx),
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut Context,
        instances: &[Quad],
        mut bounds: Rectangle<u32>,
        viewport: &Viewport,
    ) {
        let target_height = viewport.physical_height();
        bounds.height = bounds.height.min(target_height);

        ctx.apply_scissor_rect(
            bounds.x as i32,
            (target_height - (bounds.y + bounds.height)) as i32, // todo
            bounds.width as i32,
            bounds.height as i32,
        );

        ctx.buffer_update(
            self.bindings.vertex_buffers[0],
            BufferSource::slice(instances),
        );

        let indices: Vec<i32> = (0..instances.len().min(MAX_QUADS) as i32)
            .flat_map(|i| [i * 4, 1 + i * 4, 2 + i * 4, 2 + i * 4, 1 + i * 4, 3 + i * 4])
            .cycle()
            .take(instances.len() * 6)
            .collect();
        ctx.buffer_update(self.bindings.index_buffer, BufferSource::slice(&indices));

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(UniformsSource::table(&Uniforms {
            transform: *viewport.projection().as_ref(),
            scale: viewport.scale_factor() as f32,
            screen_height: target_height,
            ..Default::default()
        }));

        ctx.draw(0, indices.len() as i32, instances.len() as i32);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct Uniforms {
    pub transform: [f32; 16],
    pub scale: f32,
    pub screen_height: u32,
    // Uniforms must be aligned to their largest member,
    // this uses a mat4x4<f32> which aligns to 16, so align to that
    _padding: [f32; 2],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::IDENTITY.as_ref(),
            scale: 1.0,
            screen_height: 0,
            _padding: [0.0; 2],
        }
    }
}

impl Uniforms {
    fn uniforms() -> UniformBlockLayout {
        UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("u_Transform", UniformType::Mat4),
                UniformDesc::new("u_Scale", UniformType::Float1),
                UniformDesc::new("u_ScreenHeight", UniformType::Int1),
            ],
        }
    }
}

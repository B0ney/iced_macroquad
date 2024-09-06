use bytemuck::{Pod, Zeroable};
use iced_core::{Rectangle, Transformation};
use iced_graphics::{color, Viewport};
use macroquad::math::Mat4;

use crate::mq::{self, *};

const MAX_QUADS: usize = 1000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;

/// The properties of a quad.
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Quad {
    pub color: [f32; 4],

    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The border color of the [`Quad`]
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
    fn bindings(ctx: &mut Context) -> mq::Bindings {
        // Create static buffer to store quad vertices.
        let vertices: [[f32; 2]; 4] = [
            [1.0, 1.0],   // bottom right
            [1.0, -1.0],  // top right
            [-1.0, -1.0], // top left
            [-1.0, 1.0],  // bottom left
        ];

        let vertices: [f32; 8] = bytemuck::cast(vertices);

        let quad_geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        // Create static buffer to store quad vertex indices.
        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // Create an empty, dynamic instance buffer to store quad data.
        let quad_property_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<Quad>(MAX_QUADS),
        );

        Bindings {
            vertex_buffers: vec![quad_geometry_vertex_buffer, quad_property_vertex_buffer],
            index_buffer,
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
            VertexAttribute::with_buffer("i_inst_pos", VertexFormat::Float2, 0),
            VertexAttribute::with_buffer("i_color", VertexFormat::Float4, 1),
            VertexAttribute::with_buffer("i_pos", VertexFormat::Float2, 1),
            VertexAttribute::with_buffer("i_size", VertexFormat::Float2, 1),
            VertexAttribute::with_buffer("i_border_color", VertexFormat::Float4, 1),
            VertexAttribute::with_buffer("i_border_radius", VertexFormat::Float4, 1),
            VertexAttribute::with_buffer("i_border_width", VertexFormat::Float1, 1),
            // VertexAttribute::new("i_shadow_color", VertexFormat::Float4),
            // VertexAttribute::new("i_shadow_offset", VertexFormat::Float2),
            // VertexAttribute::new("i_shadow_blur_radius", VertexFormat::Float1),
        ];

        ctx.new_pipeline(
            &[
                mq::BufferLayout::default(),
                mq::BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            attributes,
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),

                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),

                front_face_order: FrontFaceOrder::Clockwise,
                color_write: (true, true, true, true),
                primitive_type: PrimitiveType::Triangles,

                ..Default::default()
            },
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

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(UniformsSource::table(&Uniforms {
            transform: *viewport.projection().as_ref(), // see: pg 465 in learopengl
            scale: viewport.scale_factor() as f32,
            ..Default::default()
        }));

        // Resize quad instance buffer if there are more instances
        let bytes = instances.len() * size_of::<Quad>();
        if ctx.buffer_size(self.bindings.vertex_buffers[1]) < bytes {
            ctx.delete_buffer(self.bindings.vertex_buffers[1]);

            self.bindings.vertex_buffers[1] = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Stream,
                BufferSource::empty::<Quad>(instances.len()),
            );
        }

        // Update quad buffer with quads from instances
        ctx.buffer_update(
            self.bindings.vertex_buffers[1],
            BufferSource::slice(instances),
        );

        // Clip bounds.
        ctx.apply_scissor_rect(
            bounds.x as i32,
            (target_height - (bounds.y + bounds.height)) as i32,
            bounds.width as i32,
            bounds.height as i32,
        );

        ctx.draw(0, 6, instances.len() as i32);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct Uniforms {
    pub transform: [f32; 16],
    pub scale: f32,
    // _padding: [f32; 3],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::IDENTITY.as_ref(),
            scale: 1.0,
            // _padding: [0.0; 3],
        }
    }
}

impl Uniforms {
    fn uniforms() -> UniformBlockLayout {
        UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("u_Transform", UniformType::Mat4),
                UniformDesc::new("u_Scale", UniformType::Float1),
            ],
        }
    }
}

use bytemuck::{Pod, Zeroable};
use iced_graphics::color;
use macroquad::miniquad::{self as mq, Context, UniformsSource};

pub struct Pipeline {
    pub pipeline: mq::Pipeline,
    pub bindings: mq::Bindings,
}

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

impl Pipeline {
    pub fn new(ctx: &mut Context) -> Self {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            mq::BufferType::VertexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            mq::BufferType::IndexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(&indices),
        );

        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let texture = ctx.new_texture_from_rgba8(4, 4, &pixels);

        let bindings = mq::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    mq::Backend::OpenGl => mq::ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    mq::Backend::Metal => mq::ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[mq::BufferLayout::default()],
            &[
                mq::VertexAttribute::new("in_pos", mq::VertexFormat::Float2),
                mq::VertexAttribute::new("in_uv", mq::VertexFormat::Float2),
            ],
            shader,
            mq::PipelineParams::default(),
        );
        Self { pipeline, bindings }
    }

    pub fn render(&self, ctx: &mut Context, quads: &[[f32; 2]]) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        for [x, y] in quads.iter().copied() {
            ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms { offset: (x, y) }));
            ctx.draw(0, 6, 1);
        }
    }
}

/// The properties of a quad.
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Quad {
    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The border color of the [`Quad`], in __linear RGB__.
    pub border_color: color::Packed,

    /// The border radii of the [`Quad`].
    pub border_radius: [f32; 4],

    /// The border width of the [`Quad`].
    pub border_width: f32,

    /// The shadow color of the [`Quad`].
    pub shadow_color: color::Packed,

    /// The shadow offset of the [`Quad`].
    pub shadow_offset: [f32; 2],

    /// The shadow blur radius of the [`Quad`].
    pub shadow_blur_radius: f32,
}

mod shader {
    use macroquad::miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos + offset, 0, 1);
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Uniforms
    {
        float2 offset;
    };

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float2 in_uv    [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float2 uv       [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(
      Vertex v [[stage_in]], 
      constant Uniforms& uniforms [[buffer(0)]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy + uniforms.offset, 0.0, 1.0);
        out.uv = v.in_uv;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
    {
        return tex.sample(texSmplr, in.uv);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}

pub mod state;

use macroquad::miniquad::Context;

use super::{layer::Layer, quad, text, Canvas};

/// QuadGl rendering engine
pub struct Engine {
    pub(crate) text_pipeline: text::Pipeline,
    pub(crate) quad_pipeline: quad::Pipeline,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            text_pipeline: text::Pipeline::new(),
            quad_pipeline: quad::Pipeline::new(ctx),
        }
    }

    pub fn present(&mut self, gl: &mut Context, canvas: &mut Canvas) {
        for Layer {
            bounds,
            quads,
            text,
        } in canvas.layers.iter_mut()
        {
            if !quads.is_empty() {
                self.quad_pipeline.render(gl, &quads);
            }

            // draw tris

            // draw primitives

            // draw images

            // draw text
        }

        gl.commit_frame();
    }
}

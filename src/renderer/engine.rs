use macroquad::miniquad as mq;
use mq::{Context, PassAction};

use super::{layer::Layer, quad, text, Canvas};

/// Rendering engine
pub struct Engine {
    pub(crate) text_pipeline: text::Pipeline,
    pub(crate) quad_pipeline: quad::Pipeline,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            text_pipeline: text::Pipeline::new(ctx),
            quad_pipeline: quad::Pipeline::new(ctx),
        }
    }

    pub fn present(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        ctx.begin_default_pass(PassAction::Nothing);

        for Layer {
            bounds,
            quads,
            text,
        } in canvas.layers.iter_mut()
        {
            if !quads.is_empty() {
                self.quad_pipeline.render(ctx, &quads);
            }

            // draw tris

            // draw primitives

            // draw images

            // draw text
        }

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

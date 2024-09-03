use iced_core::Rectangle;
use iced_graphics::Viewport;

use crate::mq::{Context, PassAction};

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

    pub fn present(&mut self, ctx: &mut Context, canvas: &mut Canvas, viewport: &Viewport) {
        ctx.begin_default_pass(PassAction::Nothing);

        for Layer {
            bounds,
            quads,
            text,
        } in canvas.layers.iter_mut()
        {
            // TODO
            let bounds = Rectangle {
                x: bounds.x as u32,
                y: bounds.y as u32,
                width: bounds.width as u32,
                height: bounds.height as u32,
            };

            if !quads.is_empty() {
                self.quad_pipeline.render(ctx, &quads, bounds, &viewport);
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

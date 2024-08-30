pub mod state;

use macroquad::miniquad::Context;

use super::{quad, text};

/// QuadGl rendering engine
pub struct Engine {
    pub(crate) text_pipeline: text::Pipeline,
    pub(crate) quad_pipeline: quad::Pipeline,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            text_pipeline: text::Pipeline::new(),
            quad_pipeline: quad::Pipeline::new(),
        }
    }
}

use iced_core::{renderer, Background, Rectangle, Transformation};
use iced_graphics::layer;

pub type Stack = layer::Stack<Layer>;
use super::{quad, text};

#[derive(Debug)]
pub struct Layer {
    pub bounds: Rectangle,
    pub quads: (),
    pub text: (),
}

impl Layer {
    pub fn draw_quad(
        &mut self,
        quad: renderer::Quad,
        background: Background,
        transformation: Transformation,
    ) {
    }
}

impl iced_graphics::Layer for Layer {
    fn with_bounds(bounds: Rectangle) -> Self {
        Self {
            bounds,
            ..Default::default()
        }
    }

    fn flush(&mut self) {
    }

    fn resize(&mut self, bounds: Rectangle) {
        self.bounds = bounds
    }

    fn reset(&mut self) {
        self.bounds = Rectangle::INFINITE;
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            bounds: Rectangle::INFINITE,
            quads: Default::default(),
            text: Default::default(),
        }
    }
}

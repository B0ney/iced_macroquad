use iced_core::{renderer, Background, Color, Rectangle, Transformation};
use iced_graphics::layer;

pub type Stack = layer::Stack<Layer>;
use super::{quad, text};

#[derive(Debug)]
pub struct Layer {
    pub bounds: Rectangle,
    pub quads: Vec<quad::Quad>,
    pub text: (),
}

impl Layer {
    pub fn draw_quad(
        &mut self,
        quad: renderer::Quad,
        background: Background,
        transformation: Transformation,
    ) {
        let bounds = quad.bounds * transformation;

        self.quads.push(quad::Quad {
            color: match background {
                Background::Color(Color { r, g, b, a }) => [r, g, b, a],
                Background::Gradient(_) => [0.0, 0.0, 0.0, 1.0], // todo: gradients
            },
            position: bounds.position().into(),
            size: bounds.size().into(),
            // border_color: quad.border.color.into_linear(),
            // border_radius: quad.border.radius.into(),
            // border_width: quad.border.width,
            // shadow_color: quad.shadow.color.into_linear(),
            // shadow_offset: quad.shadow.offset.into(),
            // shadow_blur_radius: quad.shadow.blur_radius,
        });
    }
}

impl iced_graphics::Layer for Layer {
    fn with_bounds(bounds: Rectangle) -> Self {
        Self {
            bounds,
            ..Default::default()
        }
    }

    fn flush(&mut self) {}

    fn resize(&mut self, bounds: Rectangle) {
        self.bounds = bounds
    }

    fn reset(&mut self) {
        self.bounds = Rectangle::INFINITE;
        self.quads.clear();
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

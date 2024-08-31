pub mod engine;
mod layer;
mod quad;
mod text;

use iced_core::renderer::Quad;
use iced_core::{Background, Color, Font, Pixels, Point, Rectangle, Transformation};

#[derive(Debug)]
pub struct Canvas {
    layers: layer::Stack,
    // todo: storage
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            layers: layer::Stack::new(),
        }
    }
}

impl iced_core::Renderer for Canvas {
    fn start_layer(&mut self, bounds: Rectangle) {
        self.layers.push_clip(bounds);
    }

    fn end_layer(&mut self) {
        self.layers.pop_clip();
    }

    fn start_transformation(&mut self, transformation: Transformation) {
        self.layers.push_transformation(transformation);
    }

    fn end_transformation(&mut self) {
        self.layers.pop_transformation();
    }

    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
        let (layer, transformation) = self.layers.current_mut();
        layer.draw_quad(quad, background.into(), transformation);
    }

    fn clear(&mut self) {
        self.layers.clear()
    }
}

impl iced_core::text::Renderer for Canvas {
    type Font = Font;
    type Paragraph = ();
    type Editor = ();

    const ICON_FONT: Self::Font = Font::DEFAULT;

    const CHECKMARK_ICON: char = '0';

    const ARROW_DOWN_ICON: char = '0';

    fn default_font(&self) -> Self::Font {
        Font::default()
    }

    fn default_size(&self) -> Pixels {
        Pixels(16.0)
    }

    fn fill_paragraph(
        &mut self,
        text: &Self::Paragraph,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }

    fn fill_editor(
        &mut self,
        editor: &Self::Editor,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }

    fn fill_text(
        &mut self,
        text: iced_core::Text<String, Self::Font>,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }
}

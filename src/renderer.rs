pub mod state;

use iced_core::{
    renderer::Quad, Background, Color, Font, Pixels, Point, Rectangle, Transformation,
};
use macroquad::miniquad::Context;
use state::State;

#[derive(Debug, Clone, Copy)]
pub enum DrawCommand {
    FillQuad(Quad, Background),
    StartLayer,
    EndLayer,
    StartTransformation(Transformation),
    EndTransformation,
    Clear,
}

pub struct Painter {
    commands: Vec<DrawCommand>,
    state: State,
}

impl Painter {
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            commands: Vec::new(),
            state: State::new(ctx),
        }
    }
    pub fn clear(&mut self) {
        self.commands.clear()
    }

    pub fn add(&mut self, command: DrawCommand) {
        self.commands.push(command)
    }
}

impl iced_core::Renderer for Painter {
    fn start_layer(&mut self, bounds: Rectangle) {
        self.add(DrawCommand::StartLayer)
    }

    fn end_layer(&mut self) {
        self.add(DrawCommand::EndLayer)
    }

    fn start_transformation(&mut self, transformation: iced_core::Transformation) {
        self.add(DrawCommand::StartTransformation(transformation))
    }

    fn end_transformation(&mut self) {
        self.add(DrawCommand::EndTransformation)
    }

    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
        self.add(DrawCommand::FillQuad(quad, background.into()))
    }

    fn clear(&mut self) {
        self.add(DrawCommand::Clear)
    }
}

impl iced_core::text::Renderer for Painter {
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

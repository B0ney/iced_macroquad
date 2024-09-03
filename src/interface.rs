use std::marker::PhantomData;

use iced_core::mouse::{Cursor, Interaction};
use iced_core::renderer::Style;
use iced_core::{Element, Point, Size};
use iced_graphics::Viewport;
use iced_runtime::{user_interface::Cache, UserInterface};

use crate::mq::CursorIcon;

use crate::context::{global, Context};
use crate::renderer::Canvas;
use crate::{convert, Renderer};

pub struct Interface<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    ui_cache: Option<Cache>,
    canvas: Canvas,
    theme: Theme,
    interacted: bool,
    _message: PhantomData<Message>,
}

impl<Message, Theme: Default> Interface<Message, Theme> {
    pub fn new() -> Self {
        Self::new_themed(Theme::default())
    }
}

impl<Message, Theme> Interface<Message, Theme> {
    pub fn new_themed(theme: Theme) -> Self {
        Self {
            in_events: Vec::new(),
            ui_cache: None,
            canvas: Canvas::new(),
            theme,
            interacted: false,
            _message: PhantomData,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    /// Interact with, and view the UI. All interactions will be pushed to messages.
    pub fn view<'a, E>(&mut self, messages: &mut Vec<Message>, ui: E)
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        global::iced_ctx_mut(|ctx| self.present(ctx, messages, ui.into()));
    }

    fn present(
        &mut self,
        ctx: &mut Context,
        messages: &mut Vec<Message>,
        ui: Element<'_, Message, Theme, Renderer>,
    ) {
        // Build the interface.
        let mut interface = UserInterface::build(
            ui,
            ctx.viewport().logical_size(),
            self.ui_cache.take().unwrap_or_default(),
            &mut self.canvas,
        );

        // Fetch all external inputs.
        self.in_events.clear();
        ctx.read_events(&mut self.in_events);

        // Update the interface by processing the events.
        let cursor = Cursor::Available(Point::from(ctx.mouse_position()));
        let (_, _statuses) = interface.update(
            &self.in_events,
            cursor,
            &mut self.canvas,
            &mut ctx.clipboard,
            messages,
        );

        // Draw the interface onto the canvas.
        let interaction = interface.draw(&mut self.canvas, &self.theme, &Style::default(), cursor);

        // Update mouse cursor.
        if interaction == Interaction::None {
            if self.interacted {
                self.interacted = false;
                ctx.set_mouse_icon(CursorIcon::Default);
            }
        } else {
            ctx.set_mouse_icon(convert::cursor_icon(interaction));
            self.interacted = true;
        }

        // Cache the interface for reuse the next time view is called.
        self.ui_cache = Some(interface.into_cache());

        // Render what's drawn on the canvas to the screen.
        ctx.render(&mut self.canvas);
    }
}

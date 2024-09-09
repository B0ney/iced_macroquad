use std::marker::PhantomData;

use iced_core::mouse::{Cursor, Interaction};
use iced_core::renderer::Style;
use iced_core::{Element, Point};
use iced_runtime::{user_interface::Cache, UserInterface};

use crate::iced::Renderer;
use crate::mq::CursorIcon;

use crate::context::{global, Context};
use crate::convert;

pub struct Interface<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    ui_cache: Option<Cache>,
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
        let viewport = ctx.viewport();
        // Build the interface.
        let mut interface = UserInterface::build(
            ui,
            viewport.logical_size(),
            self.ui_cache.take().unwrap_or_default(),
            &mut ctx.renderer,
        );

        // Fetch all external inputs.
        self.in_events.clear();
        ctx.read_events(&mut self.in_events);

        // Update the interface by processing the events.
        let cursor = Cursor::Available(Point::from(ctx.mouse_position()));
        let (_, _statuses) = interface.update(
            &self.in_events,
            cursor,
            &mut ctx.renderer,
            &mut ctx.clipboard,
            messages,
        );

        // Draw the interface onto the canvas.
        let interaction = interface.draw(&mut ctx.renderer, &self.theme, &Style::default(), cursor);

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
        ctx.present(&viewport);
    }
}

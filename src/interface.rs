use std::marker::PhantomData;

use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::{clipboard, Element, Point};
use iced_runtime::{user_interface::Cache, UserInterface};

use macroquad::input::mouse_position;
use macroquad::miniquad::window::set_mouse_cursor;

use crate::engine::{global, Engine};
use crate::event_handler::EventProxy;
use crate::{convert, Renderer};

pub struct Interface<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    ui_cache: Option<Cache>,
    theme: Theme,
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
            _message: PhantomData,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    /// Interact with the UI, sending all messages to the handler.
    ///
    /// ```rust
    /// use iced_macroquad::widget;
    ///
    /// enum Message {
    ///     Hi,
    ///     Bye
    /// }
    ///
    /// let mut messages = Vec::new();
    ///
    /// ui.interact_with(
    ///     &mut messages,
    ///     widget::button("hello").on_click(Message::Hi)
    /// );
    /// ```
    pub fn interact_with<'a, E>(&mut self, messages: &mut Vec<Message>, ui: E)
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        global::iced_ctx_mut(|ctx| self.update(ctx, messages, ui.into()));
    }

    #[must_use = "Messages should be handled."]
    pub fn interact<'a, E>(&mut self, ui: E) -> Vec<Message>
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        let mut messages = Vec::new();
        self.interact_with(&mut messages, ui);
        messages
    }

    fn update<'a>(
        &mut self,
        ctx: &mut Engine,
        messages: &mut Vec<Message>,
        ui: Element<'a, Message, Theme, Renderer>,
    ) {
        let cache = self.ui_cache.take().unwrap_or_else(Cache::new);

        let viewport = ctx.fetch_viewport();

        // Fetch all inputs
        self.in_events.clear();
        ctx.read_events(&mut EventProxy(&mut self.in_events));

        let renderer = &mut ctx.renderer;
        let cursor = Cursor::Available(Point::from(mouse_position()));

        let mut interface = UserInterface::build(ui, viewport.logical_size(), cache, renderer);

        let (_, _statuses) = interface.update(
            &self.in_events,
            cursor,
            renderer,
            &mut clipboard::Null,
            messages,
        );

        // Draw the interface
        let interaction = interface.draw(renderer, &self.theme, &Style::default(), cursor);

        // Update cursor icon.
        set_mouse_cursor(convert::cursor_icon(interaction));

        self.ui_cache = Some(interface.into_cache());
    }

    /// Present the UI
    pub fn present(&mut self) {
        global::iced_ctx_mut(|ctx| {
            ctx.renderer.present()
        })
    }
}

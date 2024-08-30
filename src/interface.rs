use std::marker::PhantomData;

use iced_core::mouse::{Cursor, Interaction};
use iced_core::renderer::Style;
use iced_core::{clipboard, Element, Point, Size};
use iced_graphics::Viewport;
use iced_runtime::{user_interface::Cache, UserInterface};

use macroquad::input::mouse_position;
use macroquad::miniquad::window::{dpi_scale, screen_size, set_mouse_cursor};

use crate::context::{global, Context};
use crate::renderer::Canvas;
use crate::{convert, Renderer};

pub struct Interface<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    ui_cache: Option<Cache>,
    canvas: Canvas,
    theme: Theme,
    mouse_icon: Interaction,
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
            mouse_icon: Interaction::None,
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

    fn update(
        &mut self,
        ctx: &mut Context,
        messages: &mut Vec<Message>,
        ui: Element<'_, Message, Theme, Renderer>,
    ) {
        // Fetch all external inputs
        self.in_events.clear();
        ctx.read_events(&mut self.in_events);

        // Build the interface
        let mut interface = UserInterface::build(
            ui,
            fetch_viewport().logical_size(),
            self.ui_cache.take().unwrap_or_default(),
            &mut self.canvas,
        );

        // Update the interface
        let cursor = fetch_cursor();
        let (_, _statuses) = interface.update(
            &self.in_events,
            cursor,
            &mut self.canvas,
            &mut clipboard::Null,
            messages,
        );

        // Draw the interface, update the mouse icon for when we present the ui.
        self.mouse_icon = interface.draw(&mut self.canvas, &self.theme, &Style::default(), cursor);

        self.ui_cache = Some(interface.into_cache());
    }

    /// Interacting with the UI will update the mouse icon
    pub fn update_cursor(&self) {
        set_mouse_cursor(convert::cursor_icon(self.mouse_icon));
    }

    /// Present the UI
    pub fn present(&mut self) {
        global::iced_ctx_mut(|ctx| self.canvas.present(&mut ctx.engine))
    }
}

fn fetch_viewport() -> Viewport {
    let (width, height) = screen_size();
    Viewport::with_physical_size(Size::new(width as u32, height as u32), dpi_scale() as f64)
}

fn fetch_cursor() -> Cursor {
    Cursor::Available(Point::from(mouse_position()))
}

use std::marker::PhantomData;

use crate::convert;
use crate::event_handler::{EventProxy, EventProxyHandler};

use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::{clipboard, Element, Point, Size};
use iced_graphics::Viewport;
use iced_runtime::{user_interface::Cache, UserInterface};

use macroquad::{input::mouse_position, window::screen_dpi_scale};
use miniquad::window::{screen_size, set_mouse_cursor};

struct Engine {
    input_subscriber_id: usize,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
        }
    }

    pub fn read_events<T: EventProxyHandler>(&self, proxy: &mut EventProxy<T>) {
        macroquad::input::utils::repeat_all_miniquad_input(proxy, self.input_subscriber_id);
    }
}

mod global {
    use std::{cell::RefCell, sync::Once};

    use crate::interface::Engine;

    thread_local! {
        static ICED_CONTEXT: RefCell<Engine> = init_single_thread(|| RefCell::new(Engine::new()));
    }

    fn init_single_thread<T>(init: impl FnOnce() -> T) -> T {
        try_init_single_thread(init).expect("Already initialized from another thread.")
    }

    fn try_init_single_thread<T>(init: impl FnOnce() -> T) -> Option<T> {
        static ONCE: Once = Once::new();
        let mut obj = None;
        ONCE.call_once(|| obj = Some(init()));
        obj
    }

    pub fn iced_ctx_mut<T>(f: impl FnOnce(&mut Engine) -> T) -> T {
        ICED_CONTEXT.with_borrow_mut(f)
    }

    pub fn iced_ctx<T>(f: impl FnOnce(&Engine) -> T) -> T {
        ICED_CONTEXT.with_borrow(f)
    }
}

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
    pub fn interact_with<'a, E>(&mut self, mut messages: &mut Vec<Message>, ui: E)
    where
        E: Into<Element<'a, Message, Theme, ()>>,
    {
        let cache = self.ui_cache.take().unwrap_or_else(Cache::new);

        let renderer = &mut (); // TODO

        let (width, height) = screen_size();
        let viewport = Viewport::with_physical_size(
            Size::new(width as u32, height as u32),
            screen_dpi_scale() as f64,
        );

        let mut interface = UserInterface::build(ui, viewport.logical_size(), cache, renderer);

        // Fetch all inputs
        self.in_events.clear();
        global::iced_ctx(|ctx| ctx.read_events(&mut EventProxy(&mut self.in_events)));

        let cursor = Cursor::Available(Point::from(mouse_position()));

        let (_, _statuses) = interface.update(
            &self.in_events,
            cursor,
            renderer,
            &mut clipboard::Null,
            &mut messages,
        );

        // Draw the interface
        let interaction = interface.draw(renderer, &self.theme, &Style::default(), cursor);
        
        // Update cursor icon.
        set_mouse_cursor(convert::cursor_icon(interaction));

        self.ui_cache = Some(interface.into_cache());
    }

    #[must_use = "Messages should be handled."]
    pub fn interact<'a, E>(&mut self, ui: E) -> Vec<Message>
    where
        E: Into<Element<'a, Message, Theme, ()>>,
    {
        let mut messages = Vec::new();
        self.interact_with(&mut messages, ui);
        messages
    }

    /// Present the UI
    pub fn present(&mut self) {}
}

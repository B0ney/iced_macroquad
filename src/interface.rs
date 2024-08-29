use std::marker::PhantomData;

use crate::event_handler::{EventProxy, EventProxyHandler};

use iced_core::{clipboard, mouse::Cursor, Element, Size};
use iced_graphics::Viewport;
use iced_runtime::{
    user_interface::{self, Cache},
    UserInterface,
};

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

pub struct Iced<Message, Theme = iced_core::Theme> {
    in_events: Vec<iced_core::Event>,
    ui_cache: Option<Cache>,

    _message: PhantomData<Message>,
    _theme: PhantomData<Theme>,
}

impl<Message, Theme> Iced<Message, Theme> {
    pub fn new() -> Self {
        Self {
            in_events: Vec::new(),
            ui_cache: None,
            _message: PhantomData,
            _theme: PhantomData,
        }
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

        let renderer = &mut ();

        // todo: fetch size
        let viewport = Viewport::with_physical_size(Size::new(640, 480), 1.0);
        let mut interface = UserInterface::build(ui, viewport.logical_size(), cache, renderer);

        // fetch all inputs
        global::iced_ctx(|ctx| ctx.read_events(&mut EventProxy(&mut self.in_events)));

        let (_, _statuses) = interface.update(
            &self.in_events,
            Cursor::Unavailable,
            renderer,
            &mut clipboard::Null,
            &mut messages,
        );

        // TODO: draw interface

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

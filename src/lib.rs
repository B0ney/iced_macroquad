pub mod convert;
pub mod event;

use std::marker::PhantomData;

use event::EventChannel;
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
    fn ui() {
        // macroquad::input::utils
    }
}

mod global {
    use std::{cell::RefCell, sync::Once};

    use crate::Engine;

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

trait MessageHandler<Message> {
    fn add(&mut self, message: Message);
}

impl<F, Message> MessageHandler<Message> for F
where
    F: FnMut(Message),
{
    fn add(&mut self, message: Message) {
        self(message)
    }
}

impl<Message> MessageHandler<Message> for &mut Vec<Message> {
    fn add(&mut self, message: Message) {
        self.push(message)
    }
}

struct Iced<Message, Theme> {
    in_events: Vec<iced_core::Event>,
    messages: Vec<Message>,
    ui_cache: Option<Cache>,

    _theme: PhantomData<Theme>,
}

impl<Message, Theme> Iced<Message, Theme> {
    pub fn new() -> Self {
        Self {
            in_events: Vec::new(),
            messages: Vec::new(),
            ui_cache: None,
            _theme: PhantomData,
        }
    }

    pub fn interact_into(
        &mut self,
        mut handler: impl MessageHandler<Message>,
        ui: Element<'_, Message, Theme, ()>,
    ) {
        let cache = self.ui_cache.take().unwrap_or_else(Cache::new);

        let mut viewport = Viewport::with_physical_size(Size::new(640, 480), 1.0); // todo
        let mut interface = UserInterface::build(ui, viewport.logical_size(), cache, &mut ());

        // fetch all input
        macroquad::input::utils::repeat_all_miniquad_input(
            &mut EventChannel(&mut self.in_events),
            global::iced_ctx(|e| e.input_subscriber_id),
        );

        let (_, _statuses) = interface.update(
            &self.in_events,
            Cursor::Unavailable,
            &mut (),
            &mut clipboard::Null,
            &mut self.messages,
        );

        for message in self.messages.drain(..) {
            handler.add(message);
        }

        // draw interface

        self.ui_cache = Some(interface.into_cache());
    }

    pub fn interact(&mut self, ui: Element<'_, Message, Theme, ()>) -> Vec<Message> {
        let mut messages = Vec::new();
        self.interact_into(&mut messages, ui);
        messages
    }

    pub fn present(&mut self) {}
}

#[test]
fn te() {
    let a = global::iced_ctx_mut(|iced| 1);

    println!("Hello, world!");
}

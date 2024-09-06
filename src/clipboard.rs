use crate::mq::window::{clipboard_get, clipboard_set};

#[derive(Default)]
pub struct Clipboard;

impl iced_core::Clipboard for Clipboard {
    fn read(&self, _kind: iced_core::clipboard::Kind) -> Option<String> {
        clipboard_get()
    }

    fn write(&mut self, _kind: iced_core::clipboard::Kind, contents: String) {
        clipboard_set(&contents)
    }
}

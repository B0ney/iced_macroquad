mod context;
mod convert;
mod event_handler;
mod interface;
mod clipboard;

use macroquad;
use macroquad::miniquad as mq;

pub use iced_widget as widget;

pub type Renderer = iced_tiny_skia::Renderer;

pub use interface::Interface;

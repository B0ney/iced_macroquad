mod clipboard;
mod context;
mod convert;
mod event_handler;
mod interface;

use macroquad;
use macroquad::miniquad as mq;

pub use iced_widget as widget;

pub type Renderer = iced_tiny_skia::Renderer<iced_tiny_skia::color_profile::RGBA>;

pub use interface::Interface;

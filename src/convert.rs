use macroquad::miniquad::{self as mq, CursorIcon};

use iced_core::keyboard::{key::Named, Key, Location, Modifiers};
use iced_core::mouse::{Button, Interaction};
use iced_core::{touch, Point};

/// mq sends special keys (backspace, delete, F1, ...) as characters.
/// Ignore those.
/// We also ignore '\r', '\n', '\t'.
/// Newlines are handled by the `Key::Enter` event.
pub fn is_printable_char(chr: char) -> bool {
    #![allow(clippy::manual_range_contains)]

    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    !is_in_private_use_area && !chr.is_ascii_control()
}

pub fn mouse_button(mb: mq::MouseButton) -> Option<Button> {
    match mb {
        mq::MouseButton::Left => Some(Button::Left),
        mq::MouseButton::Middle => Some(Button::Middle),
        mq::MouseButton::Right => Some(Button::Right),
        mq::MouseButton::Unknown => None,
    }
}

fn char(s: &str) -> Key {
    Key::Character(s.into())
}

fn named(n: Named) -> Key {
    Key::Named(n)
}

pub fn key_mod(md: mq::KeyMods) -> Modifiers {
    let flag = |modifier: Modifiers, set: bool| -> Modifiers {
        match set {
            true => modifier,
            false => Modifiers::empty(),
        }
    };

    Modifiers::empty()
        | flag(Modifiers::SHIFT, md.shift)
        | flag(Modifiers::CTRL, md.ctrl)
        | flag(Modifiers::ALT, md.alt)
        | flag(Modifiers::LOGO, md.logo)
}

pub fn key(key: mq::KeyCode) -> (Key, Location) {
    let mut location = Location::Standard;
    let l = &mut location;

    let char_numpad = |key: &str, location: &mut Location| -> Key {
        *location = Location::Numpad;
        char(key)
    };

    let named_numpad = |n: Named, location: &mut Location| -> Key {
        *location = Location::Numpad;
        named(n)
    };

    let named_left = |n: Named, location: &mut Location| -> Key {
        *location = Location::Left;
        named(n)
    };

    let named_right = |n: Named, location: &mut Location| -> Key {
        *location = Location::Right;
        named(n)
    };

    let key = match key {
        mq::KeyCode::Space => named(Named::Space),
        mq::KeyCode::Apostrophe => char("'"),
        mq::KeyCode::Comma => char(","),
        mq::KeyCode::Minus => char("-"),
        mq::KeyCode::Period => char("."),
        mq::KeyCode::Slash => char("/"),

        mq::KeyCode::Key0 => char("0"),
        mq::KeyCode::Key1 => char("1"),
        mq::KeyCode::Key2 => char("2"),
        mq::KeyCode::Key3 => char("3"),
        mq::KeyCode::Key4 => char("4"),
        mq::KeyCode::Key5 => char("5"),
        mq::KeyCode::Key6 => char("6"),
        mq::KeyCode::Key7 => char("7"),
        mq::KeyCode::Key8 => char("8"),
        mq::KeyCode::Key9 => char("9"),
        mq::KeyCode::Semicolon => char(";"),
        mq::KeyCode::Equal => char(","),
        mq::KeyCode::A => char("a"),
        mq::KeyCode::B => char("b"),
        mq::KeyCode::C => char("c"),
        mq::KeyCode::D => char("d"),
        mq::KeyCode::E => char("e"),
        mq::KeyCode::F => char("f"),
        mq::KeyCode::G => char("g"),
        mq::KeyCode::H => char("h"),
        mq::KeyCode::I => char("i"),
        mq::KeyCode::J => char("j"),
        mq::KeyCode::K => char("k"),
        mq::KeyCode::L => char("l"),
        mq::KeyCode::M => char("m"),
        mq::KeyCode::N => char("n"),
        mq::KeyCode::O => char("o"),
        mq::KeyCode::P => char("p"),
        mq::KeyCode::Q => char("q"),
        mq::KeyCode::R => char("r"),
        mq::KeyCode::S => char("s"),
        mq::KeyCode::T => char("t"),
        mq::KeyCode::U => char("u"),
        mq::KeyCode::V => char("v"),
        mq::KeyCode::W => char("w"),
        mq::KeyCode::X => char("x"),
        mq::KeyCode::Y => char("y"),
        mq::KeyCode::Z => char("z"),

        mq::KeyCode::LeftBracket => char("("),
        mq::KeyCode::Backslash => char("\\"),
        mq::KeyCode::RightBracket => char(")"),

        mq::KeyCode::Escape => named(Named::Escape),
        mq::KeyCode::Enter => named(Named::Enter),
        mq::KeyCode::Tab => named(Named::Tab),
        mq::KeyCode::Backspace => named(Named::Backspace),
        mq::KeyCode::Insert => named(Named::Insert),
        mq::KeyCode::Delete => named(Named::Delete),
        mq::KeyCode::Right => named(Named::ArrowRight),
        mq::KeyCode::Left => named(Named::ArrowLeft),
        mq::KeyCode::Down => named(Named::ArrowDown),
        mq::KeyCode::Up => named(Named::ArrowUp),
        mq::KeyCode::PageUp => named(Named::PageUp),
        mq::KeyCode::PageDown => named(Named::PageDown),
        mq::KeyCode::Home => named(Named::Home),
        mq::KeyCode::End => named(Named::End),
        mq::KeyCode::CapsLock => named(Named::CapsLock),
        mq::KeyCode::ScrollLock => named(Named::ScrollLock),
        mq::KeyCode::NumLock => named(Named::NumLock),
        mq::KeyCode::PrintScreen => named(Named::PrintScreen),
        mq::KeyCode::Pause => named(Named::MediaPause),
        mq::KeyCode::F1 => named(Named::F1),
        mq::KeyCode::F2 => named(Named::F2),
        mq::KeyCode::F3 => named(Named::F3),
        mq::KeyCode::F4 => named(Named::F4),
        mq::KeyCode::F5 => named(Named::F5),
        mq::KeyCode::F6 => named(Named::F6),
        mq::KeyCode::F7 => named(Named::F7),
        mq::KeyCode::F8 => named(Named::F8),
        mq::KeyCode::F9 => named(Named::F9),
        mq::KeyCode::F10 => named(Named::F10),
        mq::KeyCode::F11 => named(Named::F11),
        mq::KeyCode::F12 => named(Named::F12),
        mq::KeyCode::F13 => named(Named::F13),
        mq::KeyCode::F14 => named(Named::F14),
        mq::KeyCode::F15 => named(Named::F15),
        mq::KeyCode::F16 => named(Named::F16),
        mq::KeyCode::F17 => named(Named::F17),
        mq::KeyCode::F18 => named(Named::F18),
        mq::KeyCode::F19 => named(Named::F19),
        mq::KeyCode::F20 => named(Named::F20),
        mq::KeyCode::F21 => named(Named::F21),
        mq::KeyCode::F22 => named(Named::F22),
        mq::KeyCode::F23 => named(Named::F23),
        mq::KeyCode::F24 => named(Named::F24),
        mq::KeyCode::F25 => named(Named::F25),

        mq::KeyCode::Kp0 => char_numpad("0", l),
        mq::KeyCode::Kp1 => char_numpad("1", l),
        mq::KeyCode::Kp2 => char_numpad("2", l),
        mq::KeyCode::Kp3 => char_numpad("3", l),
        mq::KeyCode::Kp4 => char_numpad("4", l),
        mq::KeyCode::Kp5 => char_numpad("5", l),
        mq::KeyCode::Kp6 => char_numpad("6", l),
        mq::KeyCode::Kp7 => char_numpad("7", l),
        mq::KeyCode::Kp8 => char_numpad("8", l),
        mq::KeyCode::Kp9 => char_numpad("9", l),

        mq::KeyCode::KpDecimal => char_numpad(".", l),
        mq::KeyCode::KpDivide => char_numpad("/", l),
        mq::KeyCode::KpMultiply => char_numpad("*", l),
        mq::KeyCode::KpSubtract => char_numpad("-", l),
        mq::KeyCode::KpAdd => char_numpad("+", l),
        mq::KeyCode::KpEnter => named_numpad(Named::Enter, l),
        mq::KeyCode::KpEqual => char_numpad("=", l),
        mq::KeyCode::LeftShift => named_left(Named::Shift, l),
        mq::KeyCode::LeftControl => named_left(Named::Control, l),
        mq::KeyCode::LeftAlt => named_left(Named::Alt, l),
        mq::KeyCode::LeftSuper => named_left(Named::Super, l),
        mq::KeyCode::RightShift => named_right(Named::Shift, l),
        mq::KeyCode::RightControl => named_right(Named::Control, l),
        mq::KeyCode::RightAlt => named_right(Named::Alt, l),
        mq::KeyCode::RightSuper => named_right(Named::Super, l),
        mq::KeyCode::Unknown => Key::Unidentified,
        _ => Key::Unidentified,
    };

    (key, location)
}

pub fn touch(phase: mq::TouchPhase, id: u64, x: f32, y: f32) -> touch::Event {
    let id = touch::Finger(id);
    let position = Point::new(x, y);

    match phase {
        mq::TouchPhase::Started => touch::Event::FingerPressed { id, position },
        mq::TouchPhase::Moved => touch::Event::FingerMoved { id, position },
        mq::TouchPhase::Ended => touch::Event::FingerLifted { id, position },
        mq::TouchPhase::Cancelled => touch::Event::FingerLost { id, position },
    }
}

pub fn cursor_icon(icon: Interaction) -> CursorIcon {
    match icon {
        Interaction::None => CursorIcon::Default,
        Interaction::Idle => CursorIcon::Default,
        Interaction::Pointer => CursorIcon::Pointer,
        Interaction::Grab => CursorIcon::Move,
        Interaction::Text => CursorIcon::Text,
        Interaction::Crosshair => CursorIcon::Crosshair,
        Interaction::Working => CursorIcon::Wait,
        Interaction::ResizingHorizontally => CursorIcon::EWResize,
        Interaction::ResizingVertically => CursorIcon::NSResize,
        Interaction::NotAllowed => CursorIcon::NotAllowed,
        Interaction::ZoomIn | Interaction::Grabbing => CursorIcon::Default,
    }
}

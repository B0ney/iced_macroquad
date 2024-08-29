use miniquad as mq;

use iced_core::{
    keyboard::{self, key::Named, Key, Location, Modifiers},
    mouse::{self, Button},
    touch, window, Point,
};

/// miniquad sends special keys (backspace, delete, F1, ...) as characters.
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
        miniquad::MouseButton::Left => Some(Button::Left),
        miniquad::MouseButton::Middle => Some(Button::Middle),
        miniquad::MouseButton::Right => Some(Button::Right),
        miniquad::MouseButton::Unknown => None,
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
        miniquad::KeyCode::Space => named(Named::Space),
        miniquad::KeyCode::Apostrophe => char("'"),
        miniquad::KeyCode::Comma => char(","),
        miniquad::KeyCode::Minus => char("-"),
        miniquad::KeyCode::Period => char("."),
        miniquad::KeyCode::Slash => char("/"),

        miniquad::KeyCode::Key0 => char("0"),
        miniquad::KeyCode::Key1 => char("1"),
        miniquad::KeyCode::Key2 => char("2"),
        miniquad::KeyCode::Key3 => char("3"),
        miniquad::KeyCode::Key4 => char("4"),
        miniquad::KeyCode::Key5 => char("5"),
        miniquad::KeyCode::Key6 => char("6"),
        miniquad::KeyCode::Key7 => char("7"),
        miniquad::KeyCode::Key8 => char("8"),
        miniquad::KeyCode::Key9 => char("9"),
        miniquad::KeyCode::Semicolon => char(";"),
        miniquad::KeyCode::Equal => char(","),
        miniquad::KeyCode::A => char("a"),
        miniquad::KeyCode::B => char("b"),
        miniquad::KeyCode::C => char("c"),
        miniquad::KeyCode::D => char("d"),
        miniquad::KeyCode::E => char("e"),
        miniquad::KeyCode::F => char("f"),
        miniquad::KeyCode::G => char("g"),
        miniquad::KeyCode::H => char("h"),
        miniquad::KeyCode::I => char("i"),
        miniquad::KeyCode::J => char("j"),
        miniquad::KeyCode::K => char("k"),
        miniquad::KeyCode::L => char("l"),
        miniquad::KeyCode::M => char("m"),
        miniquad::KeyCode::N => char("n"),
        miniquad::KeyCode::O => char("o"),
        miniquad::KeyCode::P => char("p"),
        miniquad::KeyCode::Q => char("q"),
        miniquad::KeyCode::R => char("r"),
        miniquad::KeyCode::S => char("s"),
        miniquad::KeyCode::T => char("t"),
        miniquad::KeyCode::U => char("u"),
        miniquad::KeyCode::V => char("v"),
        miniquad::KeyCode::W => char("w"),
        miniquad::KeyCode::X => char("x"),
        miniquad::KeyCode::Y => char("y"),
        miniquad::KeyCode::Z => char("z"),

        miniquad::KeyCode::LeftBracket => char("("),
        miniquad::KeyCode::Backslash => char("\\"),
        miniquad::KeyCode::RightBracket => char(")"),

        miniquad::KeyCode::Escape => named(Named::Escape),
        miniquad::KeyCode::Enter => named(Named::Enter),
        miniquad::KeyCode::Tab => named(Named::Tab),
        miniquad::KeyCode::Backspace => named(Named::Backspace),
        miniquad::KeyCode::Insert => named(Named::Insert),
        miniquad::KeyCode::Delete => named(Named::Delete),
        miniquad::KeyCode::Right => named(Named::ArrowRight),
        miniquad::KeyCode::Left => named(Named::ArrowLeft),
        miniquad::KeyCode::Down => named(Named::ArrowDown),
        miniquad::KeyCode::Up => named(Named::ArrowUp),
        miniquad::KeyCode::PageUp => named(Named::PageUp),
        miniquad::KeyCode::PageDown => named(Named::PageDown),
        miniquad::KeyCode::Home => named(Named::Home),
        miniquad::KeyCode::End => named(Named::End),
        miniquad::KeyCode::CapsLock => named(Named::CapsLock),
        miniquad::KeyCode::ScrollLock => named(Named::ScrollLock),
        miniquad::KeyCode::NumLock => named(Named::NumLock),
        miniquad::KeyCode::PrintScreen => named(Named::PrintScreen),
        miniquad::KeyCode::Pause => named(Named::MediaPause),
        miniquad::KeyCode::F1 => named(Named::F1),
        miniquad::KeyCode::F2 => named(Named::F2),
        miniquad::KeyCode::F3 => named(Named::F3),
        miniquad::KeyCode::F4 => named(Named::F4),
        miniquad::KeyCode::F5 => named(Named::F5),
        miniquad::KeyCode::F6 => named(Named::F6),
        miniquad::KeyCode::F7 => named(Named::F7),
        miniquad::KeyCode::F8 => named(Named::F8),
        miniquad::KeyCode::F9 => named(Named::F9),
        miniquad::KeyCode::F10 => named(Named::F10),
        miniquad::KeyCode::F11 => named(Named::F11),
        miniquad::KeyCode::F12 => named(Named::F12),
        miniquad::KeyCode::F13 => named(Named::F13),
        miniquad::KeyCode::F14 => named(Named::F14),
        miniquad::KeyCode::F15 => named(Named::F15),
        miniquad::KeyCode::F16 => named(Named::F16),
        miniquad::KeyCode::F17 => named(Named::F17),
        miniquad::KeyCode::F18 => named(Named::F18),
        miniquad::KeyCode::F19 => named(Named::F19),
        miniquad::KeyCode::F20 => named(Named::F20),
        miniquad::KeyCode::F21 => named(Named::F21),
        miniquad::KeyCode::F22 => named(Named::F22),
        miniquad::KeyCode::F23 => named(Named::F23),
        miniquad::KeyCode::F24 => named(Named::F24),
        miniquad::KeyCode::F25 => named(Named::F25),

        miniquad::KeyCode::Kp0 => char_numpad("0", l),
        miniquad::KeyCode::Kp1 => char_numpad("1", l),
        miniquad::KeyCode::Kp2 => char_numpad("2", l),
        miniquad::KeyCode::Kp3 => char_numpad("3", l),
        miniquad::KeyCode::Kp4 => char_numpad("4", l),
        miniquad::KeyCode::Kp5 => char_numpad("5", l),
        miniquad::KeyCode::Kp6 => char_numpad("6", l),
        miniquad::KeyCode::Kp7 => char_numpad("7", l),
        miniquad::KeyCode::Kp8 => char_numpad("8", l),
        miniquad::KeyCode::Kp9 => char_numpad("9", l),

        miniquad::KeyCode::KpDecimal => char_numpad(".", l),
        miniquad::KeyCode::KpDivide => char_numpad("/", l),
        miniquad::KeyCode::KpMultiply => char_numpad("*", l),
        miniquad::KeyCode::KpSubtract => char_numpad("-", l),
        miniquad::KeyCode::KpAdd => char_numpad("+", l),
        miniquad::KeyCode::KpEnter => named_numpad(Named::Enter, l),
        miniquad::KeyCode::KpEqual => char_numpad("=", l),
        miniquad::KeyCode::LeftShift => named_left(Named::Shift, l),
        miniquad::KeyCode::LeftControl => named_left(Named::Control, l),
        miniquad::KeyCode::LeftAlt => named_left(Named::Alt, l),
        miniquad::KeyCode::LeftSuper => named_left(Named::Super, l),
        miniquad::KeyCode::RightShift => named_right(Named::Shift, l),
        miniquad::KeyCode::RightControl => named_right(Named::Control, l),
        miniquad::KeyCode::RightAlt => named_right(Named::Alt, l),
        miniquad::KeyCode::RightSuper => named_right(Named::Super, l),
        miniquad::KeyCode::Unknown => Key::Unidentified,
        _ => Key::Unidentified,
    };

    (key, location)
}

pub fn touch(phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) -> touch::Event {
    let id = touch::Finger(id);
    let position = Point::new(x, y);

    match phase {
        miniquad::TouchPhase::Started => touch::Event::FingerPressed { id, position },
        miniquad::TouchPhase::Moved => touch::Event::FingerMoved { id, position },
        miniquad::TouchPhase::Ended => touch::Event::FingerLifted { id, position },
        miniquad::TouchPhase::Cancelled => touch::Event::FingerLost { id, position },
    }
}

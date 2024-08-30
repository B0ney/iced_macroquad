use crate::convert;
use iced_core::{keyboard, mouse, window, Event, Point, Size};
use macroquad::miniquad;

pub trait EventProxy {
    fn add(&mut self, event: Event);
}

impl EventProxy for &mut Vec<Event> {
    fn add(&mut self, event: Event) {
        self.push(event)
    }
}

impl<F> EventProxy for F
where
    F: FnMut(Event),
{
    fn add(&mut self, event: Event) {
        self(event)
    }
}

pub struct EventProxyWrapper<T: EventProxy>(pub T);

impl<T: EventProxy> EventProxy for EventProxyWrapper<T> {
    fn add(&mut self, event: Event) {
        self.0.add(event)
    }
}

impl<T: EventProxy> miniquad::EventHandler for EventProxyWrapper<T> {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn resize_event(&mut self, width: f32, height: f32) {
        self.add(Event::Window(window::Event::Resized(Size::new(
            width, height,
        ))))
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.add(Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(x, y),
        }))
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.add(Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x, y },
        }))
    }

    fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, _x: f32, _y: f32) {
        if let Some(button) = convert::mouse_button(button) {
            self.add(Event::Mouse(mouse::Event::ButtonPressed(button)))
        }
    }

    fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, _x: f32, _y: f32) {
        if let Some(button) = convert::mouse_button(button) {
            self.add(Event::Mouse(mouse::Event::ButtonReleased(button)))
        }
    }

    fn char_event(&mut self, _character: char, _keymods: miniquad::KeyMods, _repeat: bool) {}

    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        let (key, location) = convert::key(keycode);
        self.add(Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            location,
            modifiers: convert::key_mod(keymods),
            text: None,
        }))
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        let (key, location) = convert::key(keycode);
        self.add(Event::Keyboard(keyboard::Event::KeyReleased {
            key,
            location,
            modifiers: convert::key_mod(keymods),
        }))
    }

    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.add(Event::Touch(convert::touch(phase, id, x, y)));
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}

    fn window_minimized_event(&mut self) {
        self.add(Event::Window(window::Event::Closed))
    }

    fn window_restored_event(&mut self) {}

    fn quit_requested_event(&mut self) {
        self.add(Event::Window(window::Event::CloseRequested))
    }

    fn files_dropped_event(&mut self) {}
}

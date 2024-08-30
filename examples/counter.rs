use iced_macroquad::widget::{button, row, text, Theme};
use iced_macroquad::Interface;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
enum Message {
    Add,
    Sub,
}

#[macroquad::main("macroqaud with iced")]
async fn main() {
    let mut ui = Interface::<Message>::new();
    let mut messages = Vec::new();
    let mut counter = 0;

    loop {
        ui.interact_with(
            &mut messages,
            row![
                button("Add").width(32).height(16).on_press(Message::Add),
                text(counter).width(32),
                button("Sub").width(32).height(16).on_press(Message::Sub),
            ],
        );

        for message in messages.drain(..) {
            match message {
                Message::Add => {
                    println!("Increment");
                    counter += 1
                }
                Message::Sub => {
                    println!("decrement");
                    counter -= 1
                }
            }
        }

        ui.update_cursor();
        ui.present();

        next_frame().await
    }
}

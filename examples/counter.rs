use iced_macroquad::alignment::Horizontal;
use iced_macroquad::widget::{button, column, text};
use iced_macroquad::Interface;

use iced_widget::center;
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
        for message in messages.drain(..) {
            match message {
                Message::Add => {
                    println!("Increment");
                    counter += 1
                }
                Message::Sub => {
                    println!("Decrement");
                    counter -= 1
                }
            }
        }

        clear_background(WHITE);

        ui.view(
            &mut messages,
            center(
                column![
                    button("Add +").on_press(Message::Add),
                    text(counter),
                    button("Sub -").on_press(Message::Sub),
                ]
                .align_x(Horizontal::Center),
            ),
        );

        next_frame().await
    }
}

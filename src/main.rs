use crate::{
    objects::{Camera, Grid, Object, Rectangle, RoundedRectangle, Text},
    structs::Vector2,
};
use raylib::prelude::*;

mod objects;
mod structs;

const TURKISH_ALPHABET: &str = " ABCDEFGHIİJKLMNOÖPRSŞTUÜVYZQWXYZabcdefghijklmnopqrstuvwxyzçğıöşüÇĞİÖŞÜ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

fn main() {
    let (mut rl_handle, thread) = raylib::init()
        .size(640, 480)
        .title("DoraNode beta test v1")
        .build();

    let roboto_font = rl_handle
        .load_font_ex(
            &thread,
            "resources/fonts/Roboto-Regular.ttf",
            64,
            Some(TURKISH_ALPHABET),
        )
        .expect("[-] Font yüklenemedi");

    let mut camera = Camera {
        offset: Vector2::new(320, 240, None),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    let text = Text {
        foreground_color: Color::BLACK,
        position: Vector2::zero(),
        text: "saç".to_string(),
        font: roboto_font,
        font_size: 16.0,
        z: 1,
    };

    while !rl_handle.window_should_close() {
        if rl_handle.is_window_resized() {
            camera.offset = Vector2::new(
                rl_handle.get_screen_width() / 2,
                rl_handle.get_screen_height() / 2,
                None,
            );
        }
        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut mode_camera = draw_handle.begin_mode2D(camera.clone());
            text.draw(&mut mode_camera);
        }
    }
}

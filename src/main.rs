use crate::{
    objects::{Camera, Grid, Object, Rectangle, RoundedRectangle, Text, TextBox},
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

    rl_handle.set_target_fps(120);
    rl_handle.set_exit_key(None);

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

    let mut text = TextBox {
        active: false,
        active_background_color: Color::WHEAT,
        background_color: Color::WHITE,
        border_color: Some(Color::BLACK),
        position: Vector2::zero(),
        border_thickness: Some(3),
        cursor_index: 0,
        foreground_color: Color::BLACK,
        size: Vector2::new(150, 50, None),
        text: String::new(),
        font_size: 0,
        font: roboto_font,
        scroll_offset: 0,
        cursor_blink: false,
        z: 1,
    };

    text.text = "sa".to_string();

    while !rl_handle.window_should_close() {
        if rl_handle.is_window_resized() {
            camera.offset = Vector2::new(
                rl_handle.get_screen_width() / 2,
                rl_handle.get_screen_height() / 2,
                None,
            );
        }
        text.update(&mut rl_handle, &thread, &camera);
        println!("{}:{}", text.active, text.text);
        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            text.draw(&mut mode_camera, &camera);
        }
    }
}

use crate::{
    objects::{Camera, Grid, Object, Rectangle, RoundedRectangle, Slider, TextBox, TextLabel},
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

    let mut slider: Slider<f32> = Slider {
        background_color: Some(Color::LIGHTGRAY),
        current_value: 0.0,
        foreground_color: None,
        handle_color: Color::DARKGRAY,
        max_value: 5.0,
        min_value: 0.0,
        position: Vector2::new(0, 100, None),
        size: Vector2::new(200, 20, None),
        handle_size: None,
        step: Some(1.0),
        z: 1,
    };

    let mut text: TextLabel = TextLabel {
        font: roboto_font,
        font_size: 32.0,
        foreground_color: Color::BLACK,
        text: String::new(),
        position: Vector2::new(0, 100, None),
        z: 2,
    };

    while !rl_handle.window_should_close() {
        if rl_handle.is_window_resized() {
            camera.offset = Vector2::new(
                rl_handle.get_screen_width() / 2,
                rl_handle.get_screen_height() / 2,
                None,
            );
        }
        slider.update(&mut rl_handle, &thread, &camera);
        text.update(&mut rl_handle, &thread, &camera);
        text.text = format!("Slider Value: {:.2}", slider.current_value);
        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            slider.draw(&mut mode_camera, &camera);
            text.draw(&mut mode_camera, &camera);
        }
    }
}

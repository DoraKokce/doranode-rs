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
        position: Vector2::new(50, 50, None),
        size: Vector2::new(200, 20, None),
        min_value: 0.0,
        max_value: 10.0,
        current_value: 5.0,
        background_color: Some(Color::GRAY),
        handle_color: Color::DARKGRAY,
        foreground_color: Some(Color::BLACK),
        step: None,
        z: 0,
    };

    let mut text_box = TextBox {
        position: Vector2::new(50, 100, None),
        size: Vector2::new(200, 40, None),
        text: String::new(),
        font: roboto_font,
        font_size: 24,
        active_background_color: Color::LIGHTGRAY,
        background_color: Color::WHITE,
        foreground_color: Color::BLACK,
        border_color: Some(Color::BLACK),
        border_thickness: Some(2),
        active: false,
        cursor_index: 0,
        scroll_offset: 0,
        cursor_blink: false,
        is_editable: false,
        scalable: true,
        min_size: Some(Vector2::new(100, 40, None)),
        z: 0,
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
        text_box.update(&mut rl_handle, &thread, &camera);
        text_box.text = format!("{:.2}", slider.current_value);
        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            slider.draw(&mut mode_camera, &camera);
            text_box.draw(&mut mode_camera, &camera);
        }
    }
}

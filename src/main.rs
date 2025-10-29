use std::{cell::RefCell, fs, rc::Rc};

use crate::{
    node_libary::NodeLibary,
    node_translations::NodeTranslations,
    objects::{Camera, Object},
    structs::Vector2,
};
use raylib::prelude::*;

mod node;
mod node_libary;
mod node_translations;
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

    let roboto_font = Rc::new(
        rl_handle
            .load_font_ex(
                &thread,
                "resources/fonts/Roboto-Regular.ttf",
                64,
                Some(TURKISH_ALPHABET),
            )
            .expect("[-] Font yüklenemedi"),
    );

    let node_translations = NodeTranslations::new();
    node_translations
        .borrow_mut()
        .load_from_file(
            fs::read_to_string("resources/translations/turkish.json")
                .expect("couldn't load translation"),
            "turkish",
        )
        .load_from_file(
            fs::read_to_string("resources/translations/english.json")
                .expect("couldn't load translation"),
            "english",
        );

    let mut camera = Camera {
        offset: Vector2::new(320, 240, None),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    let node_lib = NodeLibary::insert_default_nodes();
    let language = Rc::new(RefCell::new("turkish".to_string()));

    let node = node_lib
        .generate(
            "doranode:math.div",
            roboto_font.clone(),
            node_translations.clone(),
            language.clone(),
        )
        .expect("cannot load div");
    let node2 = node_lib
        .generate(
            "doranode:math.mul",
            roboto_font.clone(),
            node_translations.clone(),
            language.clone(),
        )
        .expect("cannot load mul");

    node2.borrow_mut().set_position((100, 100).into());

    while !rl_handle.window_should_close() {
        if rl_handle.is_window_resized() {
            camera.offset = Vector2::new(
                rl_handle.get_screen_width() / 2,
                rl_handle.get_screen_height() / 2,
                None,
            );
        }
        node.borrow_mut().update(&mut rl_handle, &thread, &camera);
        node2.borrow_mut().update(&mut rl_handle, &thread, &camera);

        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            node.borrow().draw(&mut mode_camera, &camera);
            node2.borrow().draw(&mut mode_camera, &camera);

            if node2.borrow().active {
                node.borrow_mut().active = false;
            }
        }
    }
}

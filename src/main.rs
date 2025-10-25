use std::{cell::RefCell, rc::Rc};

use crate::{
    node::{AnyPort, Connection, Node, Port},
    objects::{Camera, Object},
    structs::Vector2,
};
use raylib::prelude::*;

mod node;
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

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 200, None),
        "Test Node",
        roboto_font,
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
    );

    Node::add_port(
        &node,
        Box::new(Port::<i32>::new(0, None, &node)),
        "sa",
        false,
    );

    Node::add_port(
        &node,
        Box::new(Port::<i32>::new(20, None, &node)),
        "sa",
        true,
    );

    while !rl_handle.window_should_close() {
        node.borrow_mut().update(&mut rl_handle, &thread, &camera);

        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            node.borrow().draw(&mut mode_camera, &camera);
        }
    }
}

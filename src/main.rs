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
        Vector2::new(150, 80, None),
        "Add Node",
        roboto_font,
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<i32>(node, "A", false);
            let b = Node::read_typed_port::<i32>(node, "A", false);
            if a.is_none() || b.is_none() {
                return;
            }
            let a = a.unwrap();
            let b = b.unwrap();

            Node::write_typed_port::<i32>(node, "A + B", a + b, true);
            println!("{}", a + b);
        })),
    );

    Node::add_port(
        &node,
        Box::new(Port::<i32>::new(10, None, &node)),
        "A",
        false,
    );

    Node::add_port(
        &node,
        Box::new(Port::<i32>::new(30, None, &node)),
        "B",
        false,
    );

    Node::add_port(
        &node,
        Box::new(Port::<i32>::new(20, None, &node)),
        "A + B",
        true,
    );

    while !rl_handle.window_should_close() {
        if rl_handle.is_window_resized() {
            camera.offset = Vector2::new(
                rl_handle.get_screen_width() / 2,
                rl_handle.get_screen_height() / 2,
                None,
            );
        }
        node.borrow_mut().update(&mut rl_handle, &thread, &camera);
        Node::write_typed_port(&node.borrow_mut(), "A", 1, false);
        Node::write_typed_port(&node.borrow_mut(), "B", 1, false);
        println!(
            "{:?}",
            Node::read_typed_port::<i32>(&node.borrow_mut(), "A + B", true)
        );

        let mut draw_handle = rl_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        {
            let mut mode_camera = draw_handle.begin_mode2D(Camera2D::from(camera.clone()));
            node.borrow().draw(&mut mode_camera, &camera);
        }
    }
}

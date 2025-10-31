use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::{
    node::{Node, Port},
    node_libary::NodeLibary,
    node_translations::Translations,
    objects::Camera,
    structs::Vector2,
};

fn add_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<f32>(node, "A", false);
            let b = Node::read_typed_port::<f32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<f32>(node, "A + B", a.unwrap() + b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    node.size.y / 2 - 4,
                    5,
                    8,
                    node.size.y - 10,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_rectangle(
                    5,
                    node.size.y / 2 - 4,
                    node.size.y - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.add",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<f32>::new(5, None)), "A", false),
            (Box::new(Port::<f32>::new(35, None)), "B", false),
            (Box::new(Port::<f32>::new(22, None)), "A + B", true),
        ],
    );

    node
}

fn sub_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<f32>(node, "A", false);
            let b = Node::read_typed_port::<f32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<f32>(node, "A - B", a.unwrap() - b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    5,
                    node.size.y / 2 - 4,
                    node.size.y - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.sub",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<f32>::new(5, None)), "A", false),
            (Box::new(Port::<f32>::new(35, None)), "B", false),
            (Box::new(Port::<f32>::new(22, None)), "A - B", true),
        ],
    );

    node
}

fn div_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<f32>(node, "A", false);
            let b = Node::read_typed_port::<f32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<f32>(node, "A / B", a.unwrap() / b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    5,
                    node.size.y / 2 - 4,
                    node.size.y - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y - 10) / 2,
                    8,
                    6.0,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y - 10) / 2,
                    node.size.y - 10,
                    6.0,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.div",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<f32>::new(5, None)), "A", false),
            (Box::new(Port::<f32>::new(35, None)), "B", false),
            (Box::new(Port::<f32>::new(22, None)), "A / B", true),
        ],
    );

    node
}

fn mul_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<f32>(node, "A", false);
            let b = Node::read_typed_port::<f32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<f32>(node, "A * B", a.unwrap() * b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_line_ex(
                    Vector2::new(5 + node.size.y - 10, 5, None),
                    Vector2::new(5, 5 + node.size.y - 10, None),
                    8.0,
                    Color::new(100, 100, 100, 255),
                );

                draw_handle.draw_line_ex(
                    Vector2::new(5, 5, None),
                    Vector2::new(5 + node.size.y - 10, 5 + node.size.y - 10, None),
                    8.0,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.mul",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<f32>::new(5, None)), "A", false),
            (Box::new(Port::<f32>::new(35, None)), "B", false),
            (Box::new(Port::<f32>::new(22, None)), "A x B", true),
        ],
    );

    node
}

pub fn insert(libary: &mut NodeLibary) {
    libary.insert("doranode:math.add", add_ctor);
    libary.insert("doranode:math.sub", sub_ctor);
    libary.insert("doranode:math.div", div_ctor);
    libary.insert("doranode:math.mul", mul_ctor);
}

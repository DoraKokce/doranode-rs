use std::{cell::RefCell, rc::Rc};

use pyo3::{
    PyAny, Python,
    ffi::c_str,
    prelude::*,
    types::{PyAnyMethods, PyModule},
};
use raylib::prelude::*;

use crate::{
    node::{Node, Port},
    node_libary::NodeLibary,
    objects::Camera,
    structs::Vector2,
    translations::Translations,
};

fn add_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs):\n\toutputs = {}\n\toutputs[\"A + B\"] = (kwargs.get(\"inputs\")[\"A\"] or 0) + (kwargs.get(\"inputs\")[\"B\"] or 0)\n\treturn outputs"
            ),
            c_str!(""),
            c_str!(""),
        ).expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        update_fn,
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
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                5,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                35,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A + B",
                true,
                22,
            ),
        ],
    );

    node
}

fn sub_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs):\n\toutputs = {}\n\toutputs[\"A - B\"] = (kwargs.get(\"inputs\")[\"A\"] or 0) - (kwargs.get(\"inputs\")[\"B\"] or 0)\n\treturn outputs"
            ),
            c_str!(""),
            c_str!(""),
        )
        .ok()?;
        let func = module.getattr("update").ok()?;
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        update_fn,
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
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                5,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                35,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A - B",
                true,
                22,
            ),
        ],
    );

    node
}

fn div_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs):\n\toutputs = {}\n\toutputs[\"A / B\"] = (kwargs.get(\"inputs\")[\"A\"] or 0) / (kwargs.get(\"inputs\")[\"B\"] or 0)\n\treturn outputs"
            ),
            c_str!(""),
            c_str!(""),
        )
        .ok()?;
        let func = module.getattr("update").ok()?;
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        update_fn,
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
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                5,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                35,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A / B",
                true,
                22,
            ),
        ],
    );

    node
}

fn mul_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<Translations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs):\n\toutputs = {}\n\toutputs[\"A x B\"] = (kwargs.get(\"inputs\")[\"A\"] or 0) * (kwargs.get(\"inputs\")[\"B\"] or 0)\n\treturn outputs"
            ),
            c_str!(""),
            c_str!(""),
        )
        .ok()?;
        let func = module.getattr("update").ok()?;
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        update_fn,
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
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                5,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                35,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A x B",
                true,
                22,
            ),
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

use std::{cell::RefCell, rc::Rc};

use pyo3::{
    PyAny, Python,
    ffi::c_str,
    prelude::*,
    types::{PyAnyMethods, PyModule},
};
use raylib::prelude::*;

use crate::{
    colorscheme::ColorSchemes,
    node::{Node, Port},
    node_libary::NodeLibary,
    objects::Camera,
    settings::Settings,
    structs::Vector2,
    translations::Translations,
};

fn add_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: &'static str,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {\"A + B\": (kwargs[\"inputs\"][\"A\"] or 0) + (kwargs[\"inputs\"][\"B\"] or 0)}"
            ),
            c_str!(""),
            c_str!(""),
        ).expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    node.size.y as i32 / 2 - 4,
                    5,
                    8,
                    node.size.y as i32 - 10,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_rectangle(
                    5,
                    node.size.y as i32 / 2 - 4,
                    node.size.y as i32 - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.add",
        translations,
        color_schemes,
        settings,
        id,
    );

    Node::add_ports(
        &node,
        vec![
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                13,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                38,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A + B",
                true,
                25,
            ),
        ],
    );

    node
}

fn sub_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: &'static str,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {\"A - B\": (kwargs[\"inputs\"][\"A\"] or 0) - (kwargs[\"inputs\"][\"B\"] or 0)}"
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
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    5,
                    node.size.y as i32 / 2 - 4,
                    node.size.y as i32 - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.sub",
        translations,
        color_schemes,
        settings,
        id,
    );

    Node::add_ports(
        &node,
        vec![
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                13,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                38,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A - B",
                true,
                25,
            ),
        ],
    );

    node
}

fn div_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: &'static str,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {\"A / B\": (kwargs[\"inputs\"][\"A\"] or 0) / (kwargs[\"inputs\"][\"B\"] or 0)}"
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
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_rectangle(
                    5,
                    node.size.y as i32 / 2 - 4,
                    node.size.y as i32 - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y as i32 - 10) / 2,
                    8,
                    60.0,
                    Color::new(100, 100, 100, 255),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y as i32 - 10) / 2,
                    node.size.y as i32 - 10,
                    60.0,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.div",
        translations,
        color_schemes,
        settings,
        id,
    );

    Node::add_ports(
        &node,
        vec![
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                13,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                38,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A / B",
                true,
                25,
            ),
        ],
    );

    node
}

fn mul_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: &'static str,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {\"A * B\": (kwargs[\"inputs\"][\"A\"] or 0) * (kwargs[\"inputs\"][\"B\"] or 0)}"
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
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, _: Camera| {
                draw_handle.draw_line_ex(
                    Vector2::new(5.0 + node.size.y - 10.0, 5.0, None),
                    Vector2::new(5.0, 5.0 + node.size.y - 10.0, None),
                    80.0,
                    Color::new(100, 100, 100, 255),
                );

                draw_handle.draw_line_ex(
                    Vector2::new(5.0, 5.0, None),
                    Vector2::new(5.0 + node.size.y - 10.0, 5.0 + node.size.y - 10.0, None),
                    8.0,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "doranode:math.mul",
        translations,
        color_schemes,
        settings,
        id,
    );

    Node::add_ports(
        &node,
        vec![
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A",
                false,
                13,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "B",
                false,
                38,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "A x B",
                true,
                25,
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

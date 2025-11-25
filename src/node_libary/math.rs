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

pub fn insert(libary: &mut NodeLibary) {
    libary.insert("doranode:math.add", add_ctor);
    libary.insert("doranode:math.sub", sub_ctor);
    libary.insert("doranode:math.div", div_ctor);
    libary.insert("doranode:math.mul", mul_ctor);
    libary.insert("doranode:math.sqrt", sqrt_ctor);
}

fn add_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: String,
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
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );
                draw_handle.draw_rectangle(
                    5,
                    node.size.y as i32 / 2 - 4,
                    node.size.y as i32 - 10,
                    8,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
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
    id: String,
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
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
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
    id: String,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {'A / B': (lambda a,b: a/b if b else 0)(kwargs['inputs']['A'] or 0, kwargs['inputs']['B'] or 0)}"
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
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y as i32 - 10) / 2,
                    8,
                    6.0,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );
                draw_handle.draw_circle(
                    5 + (node.size.y as i32 - 10) / 2,
                    node.size.y as i32 - 10,
                    6.0,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
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
    id: String,
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
                    8.0,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );

                draw_handle.draw_line_ex(
                    Vector2::new(5.0, 5.0, None),
                    Vector2::new(5.0 + node.size.y - 10.0, 5.0 + node.size.y - 10.0, None),
                    8.0,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
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

fn sqrt_ctor(
    font: Rc<RefCell<Font>>,
    translations: Rc<RefCell<Translations>>,
    color_schemes: Rc<RefCell<ColorSchemes>>,
    settings: Rc<RefCell<Settings>>,
    id: String,
) -> Rc<RefCell<Node>> {
    let update_fn: Option<Py<PyAny>> = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(
                "def update(**kwargs): return {\"√A\": (kwargs[\"inputs\"][\"A\"] or 0) ** 0.5}"
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
                draw_handle.draw_text_ex(
                    &*node.font.borrow(),
                    "√",
                    Vector2::new(25.0, -5.0, None),
                    60.0,
                    1.0,
                    node.color_schemes
                        .borrow()
                        .get_color(&node.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );
            },
        )),
        "doranode:math.sqrt",
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
                25,
            ),
            (
                Box::new(Port::new(Color::new(30, 30, 30, 255))),
                "√A",
                true,
                25,
            ),
        ],
    );

    node
}

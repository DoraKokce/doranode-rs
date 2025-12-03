use std::{cell::RefCell, rc::Rc};

use pyo3::{ffi::c_str, prelude::*};
use raylib::{color::Color, text::Font};

use crate::{
    colorscheme::ColorSchemes,
    node::{Node, Port},
    node_libary::NodeLibary,
    objects::TextBox,
    settings::Settings,
    structs::Vector2,
    translations::Translations,
};

pub fn insert(libary: &mut NodeLibary) {
    libary.insert("doranode:io.input_int", input_int_ctor);
    libary.insert("doranode:io.input_float", input_float_ctor);
    libary.insert("doranode:io.input_str", input_str_ctor);

    libary.insert("doranode:io.output", output_ctor);
}

fn input_int_ctor(
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
                 "def update(**kwargs):\n\ttry: return {\"\": int(kwargs[\"components\"][\"textbox\"].get_property(\"text\") or \"0.0\")}\n\texcept ValueError: return {\"\": 0.0}"
            ),
            c_str!(""),
            c_str!(""),
        )
        .expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        None,
        "doranode:io.input_int",
        translations,
        color_schemes,
        settings,
        id,
        true,
    );

    Node::add_component(
        &node,
        "textbox".to_string(),
        Box::new(TextBox::new(
            Vector2::zero(),
            Vector2::new(150.0, 50.0, None),
            Color::new(80, 80, 80, 255),
            Color::new(80, 80, 80, 255),
            Color::WHITE,
            None,
            None,
            font.clone(),
            true,
            true,
            Some(Vector2::new(150.0, 50.0, None)),
            Some(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-']),
            1,
        )),
        None,
    );

    Node::add_port(
        &node,
        Box::new(Port::new(Color::new(30, 30, 30, 255))),
        "",
        true,
        25,
    );

    node
}

fn input_str_ctor(
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
                "def update(**kwargs): return {\"\": kwargs[\"components\"][\"textbox\"].get_property(\"text\") or \"\"}"
            ),
            c_str!(""),
            c_str!(""),
        )
        .expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        None,
        "doranode:io.input_str",
        translations,
        color_schemes,
        settings,
        id,
        true,
    );

    Node::add_component(
        &node,
        "textbox".to_string(),
        Box::new(TextBox::new(
            Vector2::zero(),
            Vector2::new(150.0, 50.0, None),
            Color::new(80, 80, 80, 255),
            Color::new(80, 80, 80, 255),
            Color::WHITE,
            None,
            None,
            font.clone(),
            true,
            true,
            Some(Vector2::new(150.0, 50.0, None)),
            None,
            1,
        )),
        None,
    );

    Node::add_port(
        &node,
        Box::new(Port::new(Color::new(30, 30, 30, 255))),
        "",
        true,
        25,
    );

    node
}

fn input_float_ctor(
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
                "def update(**kwargs):\n\ttry: return {\"\": float(kwargs[\"components\"][\"textbox\"].get_property(\"text\") or \"0.0\")}\n\texcept ValueError: return {\"\": 0.0}"
            ),
            c_str!(""),
            c_str!(""),
        )
        .expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        None,
        "doranode:io.input_float",
        translations,
        color_schemes,
        settings,
        id,
        true,
    );

    Node::add_component(
        &node,
        "textbox".to_string(),
        Box::new(TextBox::new(
            Vector2::zero(),
            Vector2::new(150.0, 50.0, None),
            Color::new(80, 80, 80, 255),
            Color::new(80, 80, 80, 255),
            Color::WHITE,
            None,
            None,
            font.clone(),
            true,
            true,
            Some(Vector2::new(150.0, 50.0, None)),
            Some(vec![
                '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', '-',
            ]),
            1,
        )),
        None,
    );

    Node::add_port(
        &node,
        Box::new(Port::new(Color::new(30, 30, 30, 255))),
        "",
        true,
        25,
    );

    node
}

fn output_ctor(
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
                "def update(**kwargs):\n\tkwargs[\"components\"][\"textbox\"].set_property(\"text\", str(kwargs[\"inputs\"][\"\"]))\n\treturn {}"
            ),
            c_str!(""),
            c_str!(""),
        )
        .expect("msg");
        let func = module.getattr("update").expect("msg");
        Some(func.into())
    });

    let node = Node::new(
        Vector2::zero(),
        Vector2::new(150.0, 50.0, None),
        font.clone(),
        update_fn,
        None,
        "doranode:io.output",
        translations,
        color_schemes,
        settings,
        id,
        true,
    );

    Node::add_component(
        &node,
        "textbox".to_string(),
        Box::new(TextBox::new(
            Vector2::zero(),
            Vector2::new(0.0, 0.0, None),
            Color::new(80, 80, 80, 255),
            Color::new(80, 80, 80, 255),
            Color::WHITE,
            None,
            None,
            font.clone(),
            false,
            true,
            Some(Vector2::new(150.0, 50.0, None)),
            None,
            1,
        )),
        None,
    );

    Node::add_port(
        &node,
        Box::new(Port::new(Color::new(30, 30, 30, 255))),
        "",
        false,
        25,
    );

    node
}

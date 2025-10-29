use std::{cell::RefCell, collections::HashMap, process::Output, rc::Rc};

use raylib::prelude::*;

use crate::{
    node::{Node, Port},
    node_translations::NodeTranslations,
    objects::Camera,
    structs::Vector2,
};

pub type NodeCtor =
    fn(Rc<Font>, Rc<RefCell<NodeTranslations>>, Rc<RefCell<String>>) -> Rc<RefCell<Node>>;

pub struct NodeLibary {
    nodes: HashMap<String, NodeCtor>,
}

impl NodeLibary {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, type_name: &str, ctor: NodeCtor) {
        self.nodes.insert(type_name.to_string(), ctor);
    }

    pub fn generate(
        &self,
        type_name: &str,
        font: Rc<Font>,
        translations: Rc<RefCell<NodeTranslations>>,
        language: Rc<RefCell<String>>,
    ) -> Option<Rc<RefCell<Node>>> {
        self.nodes
            .get(type_name)
            .map(|f| f(font, translations, language))
    }

    pub fn insert_default_nodes() -> Self {
        let mut libary = Self::new();

        libary.insert("math.add", add_ctor);
        libary.insert("math.sub", sub_ctor);
        libary.insert("math.div", div_ctor);

        libary
    }
}

fn add_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<NodeTranslations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let mut node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<i32>(node, "A", false);
            let b = Node::read_typed_port::<i32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<i32>(node, "A + B", a.unwrap() + b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, camera: Camera| {
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
        "math.add",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<i32>::new(5, None, &node)), "A", false),
            (Box::new(Port::<i32>::new(35, None, &node)), "B", false),
            (Box::new(Port::<i32>::new(22, None, &node)), "A + B", true),
        ],
    );

    node
}

fn sub_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<NodeTranslations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let mut node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<i32>(node, "A", false);
            let b = Node::read_typed_port::<i32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<i32>(node, "A - B", a.unwrap() - b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, camera: Camera| {
                draw_handle.draw_rectangle(
                    5,
                    node.size.y / 2 - 4,
                    node.size.y - 10,
                    8,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "math.sub",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<i32>::new(5, None, &node)), "A", false),
            (Box::new(Port::<i32>::new(35, None, &node)), "B", false),
            (Box::new(Port::<i32>::new(22, None, &node)), "A - B", true),
        ],
    );

    node
}

fn div_ctor(
    font: Rc<Font>,
    translations: Rc<RefCell<NodeTranslations>>,
    language: Rc<RefCell<String>>,
) -> Rc<RefCell<Node>> {
    let mut node = Node::new(
        Vector2::zero(),
        Vector2::new(150, 50, None),
        font.clone(),
        Color::new(60, 60, 60, 255),
        Some(Color::new(30, 30, 30, 255)),
        Color::WHITE,
        None,
        Some(Box::new(|node: &mut Node| {
            let a = Node::read_typed_port::<i32>(node, "A", false);
            let b = Node::read_typed_port::<i32>(node, "B", false);
            if a.is_none() || b.is_none() {
                return;
            }
            Node::write_typed_port::<i32>(node, "A / B", a.unwrap() - b.unwrap(), true);
        })),
        Some(Box::new(
            |node: &Node, draw_handle: &mut RaylibDrawHandle<'_>, camera: Camera| {
                draw_handle.draw_text_ex(
                    &*node.font,
                    "/",
                    Vector2::new(0, 0, None),
                    64.0,
                    1.0,
                    Color::new(100, 100, 100, 255),
                );
            },
        )),
        "math.div",
        translations,
        language,
    );

    Node::add_ports(
        &node,
        vec![
            (Box::new(Port::<i32>::new(5, None, &node)), "A", false),
            (Box::new(Port::<i32>::new(35, None, &node)), "B", false),
            (Box::new(Port::<i32>::new(22, None, &node)), "A - B", true),
        ],
    );

    node
}

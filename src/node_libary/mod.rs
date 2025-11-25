use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::{
    colorscheme::ColorSchemes,
    node::{Node, Port},
    objects::Camera,
    settings::Settings,
    structs::Vector2,
    translations::Translations,
};

mod io;
mod math;

pub type NodeCtor = fn(
    Rc<RefCell<Font>>,
    Rc<RefCell<Translations>>,
    Rc<RefCell<ColorSchemes>>,
    Rc<RefCell<Settings>>,
    &'static str,
) -> Rc<RefCell<Node>>;

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
        font: Rc<RefCell<Font>>,
        translations: Rc<RefCell<Translations>>,
        color_schemes: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        id: &'static str,
    ) -> Option<Rc<RefCell<Node>>> {
        self.nodes
            .get(type_name)
            .map(|f| f(font, translations, color_schemes, settings, id))
    }

    pub fn insert_default_nodes() -> Self {
        let mut libary = Self::new();

        math::insert(&mut libary);
        io::insert(&mut libary);

        libary
    }

    pub fn get_modules(&self) -> Vec<String> {
        let mut modules: Vec<String> = vec![];

        for node in self.nodes.keys() {
            let module = node.split(":").next();
            if module.is_some() {
                modules.push(module.unwrap().to_string());
            }
        }

        modules
    }
}

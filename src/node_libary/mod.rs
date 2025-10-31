use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::{
    node::{Node, Port},
    node_translations::Translations,
    objects::Camera,
    structs::Vector2,
};

mod math;
pub type NodeCtor =
    fn(Rc<Font>, Rc<RefCell<Translations>>, Rc<RefCell<String>>) -> Rc<RefCell<Node>>;

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
        translations: Rc<RefCell<Translations>>,
        language: Rc<RefCell<String>>,
    ) -> Option<Rc<RefCell<Node>>> {
        self.nodes
            .get(type_name)
            .map(|f| f(font, translations, language))
    }

    pub fn insert_default_nodes() -> Self {
        let mut libary = Self::new();

        math::insert(&mut libary);

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

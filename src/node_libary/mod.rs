use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::{
    colorscheme::ColorSchemes, node::Node, settings::Settings, translations::Translations,
};

mod io;
mod math;

pub type NodeCtor = fn(
    Rc<RefCell<Font>>,
    Rc<RefCell<Translations>>,
    Rc<RefCell<ColorSchemes>>,
    Rc<RefCell<Settings>>,
    String,
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
        id: String,
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

    pub fn get_hierarchy(&self) -> Vec<(String, Vec<(String, Vec<String>)>)> {
        let mut map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        for key in self.nodes.keys() {
            let (prefix, rest) = key.split_once(":").unwrap_or(("default", key));
            let (module, node) = rest.split_once(".").unwrap_or((rest, ""));

            let modules_map = map.entry(prefix.to_string()).or_default();
            let nodes_vec = modules_map.entry(module.to_string()).or_default();

            if !node.is_empty() {
                nodes_vec.push(node.to_string());
            }
        }

        for modules in map.values_mut() {
            for nodes in modules.values_mut() {
                nodes.sort();
            }
        }

        let mut prefixes: Vec<(String, HashMap<String, Vec<String>>)> = map.into_iter().collect();
        prefixes.sort_by(|a, b| a.0.cmp(&b.0));

        let mut final_vec = Vec::new();
        for (prefix, modules_map) in prefixes {
            let mut modules_vec: Vec<(String, Vec<String>)> = modules_map.into_iter().collect();
            modules_vec.sort_by(|a, b| a.0.cmp(&b.0));
            final_vec.push((prefix, modules_vec));
        }

        final_vec
    }
}

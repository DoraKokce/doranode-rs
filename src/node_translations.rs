use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Deserialize, Clone)]
pub struct NodeText {
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct NodeTranslations {
    pub translations: HashMap<String, HashMap<String, NodeText>>,
}

impl NodeTranslations {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            translations: HashMap::new(),
        }))
    }

    pub fn load_from_file(&mut self, file_contents: String, language: &str) -> &mut Self {
        let node_translations: HashMap<String, NodeText> =
            serde_json::from_str(&file_contents).expect("Couldn't parse translation JSON");

        self.translations
            .insert(language.to_string(), node_translations);
        self
    }

    pub fn get_node_translation(&self, language: &str, type_name: &str) -> NodeText {
        self.translations
            .get(language)
            .unwrap_or_else(|| self.translations.get("en").unwrap())
            .get(type_name)
            .cloned()
            .unwrap_or_else(|| NodeText {
                title: "???".to_string(),
                description: format!("This node doesn't support '{}'", type_name),
            })
    }
}

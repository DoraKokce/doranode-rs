use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Deserialize, Clone)]
pub struct NodeText {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct TranslationFile {
    nodes: Option<HashMap<String, NodeText>>,
    gui: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Translations {
    pub node_translations: Option<HashMap<String, HashMap<String, NodeText>>>,
    pub gui_translations: Option<HashMap<String, HashMap<String, String>>>,
}

impl Translations {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            node_translations: Some(HashMap::new()),
            gui_translations: Some(HashMap::new()),
        }))
    }

    pub fn load_from_file(&mut self, file_contents: String, language: &str) -> &mut Self {
        let parsed: TranslationFile =
            serde_json::from_str(&file_contents).expect("Couldn't parse translation JSON");
        println!("{:?}", self);

        if let Some(nodes) = parsed.nodes {
            self.node_translations
                .as_mut()
                .unwrap()
                .insert(language.to_string(), nodes);
        }

        if let Some(gui) = parsed.gui {
            self.gui_translations
                .as_mut()
                .unwrap()
                .insert(language.to_string(), gui);
        }

        self
    }

    pub fn get_node_translation(&self, language: &str, type_name: &str) -> NodeText {
        self.node_translations
            .as_ref()
            .unwrap()
            .get(language)
            .unwrap_or_else(|| self.node_translations.as_ref().unwrap().get("en").unwrap())
            .get(type_name)
            .cloned()
            .unwrap_or_else(|| NodeText {
                title: "???".to_string(),
                description: format!("This node doesn't support '{}'", type_name),
            })
    }

    pub fn get_gui_translation(&self, language: &str, key: &str) -> String {
        self.gui_translations
            .as_ref()
            .unwrap()
            .get(language)
            .unwrap_or_else(|| self.gui_translations.as_ref().unwrap().get("en").unwrap())
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("Missing GUI text: {}", key))
    }
}

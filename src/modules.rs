use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::text::Font;
use serde::Deserialize;
use zip::ZipArchive;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use crate::colorscheme::ColorSchemes;
use crate::node::{Node, PyNode};
use crate::settings::Settings;
use crate::structs::Vector2;
use crate::translations::Translations;

pub struct ModuleManager {
    pub modules: HashMap<String, (Module, Py<PyModule>, HashMap<String, Py<PyAny>>)>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(
        &mut self,
        path: &Path,
        translations: &mut Translations,
    ) -> Option<&mut Self> {
        let file = File::open(path).ok()?;
        let mut archive = ZipArchive::new(file).ok()?;

        let mut toml_file = archive.by_name("module.toml").ok()?;
        let mut contents = String::new();
        toml_file.read_to_string(&mut contents).ok()?;
        drop(toml_file);
        let config = toml::from_str::<Module>(&contents).ok()?;

        if config.dependincies.is_some() {
            for (dep_name, required_version) in config.dependincies.as_ref().unwrap() {
                if self.modules.get(dep_name).is_none()
                    || self.modules.get(dep_name)?.0.module.version != *required_version
                {
                    return None;
                }
            }
        }

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).ok()?;
            let file_name = file.name().to_string();

            if file_name.starts_with("translations/") && file_name.ends_with(".json") {
                let language_code = file_name
                    .trim_start_matches("translations/")
                    .trim_end_matches(".json")
                    .to_string();

                let mut translation_contents = String::new();
                file.read_to_string(&mut translation_contents).ok()?;

                translations.load_from_file(translation_contents, &language_code);
            }
        }

        let node_files: Vec<_> = (0..archive.len())
            .filter_map(|i| {
                let f = archive.by_index(i).ok()?;
                if f.name().starts_with("nodes/") && f.name().ends_with(".py") {
                    Some(f.name().to_string())
                } else {
                    None
                }
            })
            .collect();

        let _ = Python::attach(|py| -> PyResult<()> {
            let sys = py.import("sys")?;
            sys.getattr("path")?
                .call_method1("append", ("resources",))?;
            sys.getattr("path")?
                .call_method1("append", (path.to_str().unwrap(),))?;

            let main_module = py.import("module.main")?.into();

            self.modules.insert(
                config.module.name.clone(),
                (config.clone(), main_module, HashMap::new()),
            );

            for path in node_files {
                let module_name = path.trim_end_matches(".py").replace("/", ".");
                println!("{}", module_name);
                let module = py.import(&module_name).expect("hata").into();

                self.modules
                    .get_mut(&config.module.name.clone())
                    .unwrap()
                    .2
                    .insert(
                        format!(
                            "{}:{}",
                            config.module.name,
                            module_name.replace("_", ".").replace("nodes.", "")
                        ),
                        module,
                    );
            }

            Ok(())
        });

        Some(self)
    }

    pub fn generate(
        &self,
        position: Vector2,
        type_name: String,
        font: Rc<RefCell<Font>>,
        translations: Rc<RefCell<Translations>>,
        color_schemes: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        id: String,
    ) -> Option<Rc<RefCell<Node>>> {
        Python::attach(|py| -> Option<Rc<RefCell<Node>>> {
            let py_func = self
                .modules
                .iter()
                .find(|(name, _)| type_name.starts_with(*name))?
                .1
                .2
                .iter()
                .find(|(name, _)| type_name.ends_with(*name))?
                .1
                .getattr(py, "generate")
                .ok()?;

            println!("ok");

            Some(
                py_func
                    .call(py, (), None)
                    .expect("call")
                    .extract::<PyNode>(py)
                    .expect("extract")
                    .to_node(
                        position.clone(),
                        font.clone(),
                        translations.clone(),
                        color_schemes.clone(),
                        settings.clone(),
                        id.clone(),
                    ),
            )
        })
    }

    pub fn get_hierarchy(&self) -> Vec<(String, Vec<(String, Vec<String>)>)> {
        let mut temp_module_groups: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        for (module_name, (_, _, nodes_map)) in &self.modules {
            for node_path_str in nodes_map.keys() {
                let cleaned_path = node_path_str.trim_start_matches("nodes.");
                let parts: Vec<&str> = cleaned_path.split('.').collect();

                let module_entry = temp_module_groups.entry(module_name.clone()).or_default();

                match parts.as_slice() {
                    [category, node_name] => {
                        let category_entry = module_entry.entry(category.to_string()).or_default();

                        category_entry.push(node_name.to_string());
                    }
                    [node_name] => {
                        let other_category = module_entry.entry("Diğer".to_string()).or_default();
                        other_category.push(node_name.to_string());
                    }
                    _ => {
                        eprintln!("Hiyerarşi oluşturulamadı: {}", node_path_str);
                    }
                }
            }
        }

        let mut final_hierarchy: Vec<(String, Vec<(String, Vec<String>)>)> = Vec::new();

        for (module_name, categories) in temp_module_groups {
            let mut category_vec: Vec<(String, Vec<String>)> = Vec::new();

            for (category_name, node_names) in categories {
                category_vec.push((category_name, node_names));
            }

            category_vec.sort_by(|a, b| a.0.cmp(&b.0));

            final_hierarchy.push((module_name, category_vec));
        }

        final_hierarchy.sort_by(|a, b| a.0.cmp(&b.0));

        final_hierarchy
    }
}

#[derive(Deserialize, Clone)]
pub struct Module {
    pub module: ModuleConfig,
    pub dependincies: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub version: String,
}

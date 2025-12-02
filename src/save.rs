use std::{cell::RefCell, collections::HashMap, fs, num::NonZeroIsize, rc::Rc};

use rfd::FileDialog;
use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeSave {
    pub id: String,
    pub type_name: String,
    pub position: [f32; 2],
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SaveFile {
    pub project_name: String,
    pub nodes: Vec<NodeSave>,
    pub connections: Vec<String>,
    pub camera_pos: [f32; 2],
}

impl SaveFile {
    pub fn new(project_name: String) -> Self {
        SaveFile {
            project_name,
            nodes: Vec::new(),
            connections: Vec::new(),
            camera_pos: [0.0, 0.0],
        }
    }

    pub fn from(
        project_name: String,
        nodes: Vec<NodeSave>,
        connections: Vec<String>,
        camera_pos: [f32; 2],
    ) -> Self {
        SaveFile {
            project_name,
            nodes,
            connections,
            camera_pos,
        }
    }

    pub fn read() -> Option<Self> {
        if let Some(path) = FileDialog::new()
            .set_title("Open Project")
            .add_filter("DNODE File", &["dnode"])
            .set_file_name("untitled.dnode")
            .pick_file()
        {
            let content = fs::read_to_string(&path).expect("Dosya okunamadı");
            Some(serde_json::from_str::<SaveFile>(&content).expect("Failed to parse save file"))
        } else {
            None
        }
    }

    pub fn from_file(path: &str) -> Option<Self> {
        let content = fs::read_to_string(&path).expect("Dosya okunamadı");
        Some(serde_json::from_str::<SaveFile>(&content).expect("Failed to parse save file"))
    }

    pub fn write(&self) -> bool {
        FileDialog::new()
            .set_title("Save Project As")
            .add_filter("DNODE File", &["dnode"])
            .set_file_name(&format!("{}.dnode", self.project_name))
            .save_file()
            .map(|path| {
                std::fs::write(&path, serde_json::to_string_pretty(self).unwrap())
                    .expect("Failed to write save file");
                true
            })
            .unwrap_or(false)
    }
}

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub language: String,
    pub scheme: String,
    pub font: String,
    pub grid_size: [f32; 2],
    pub grid_square_size: [f32; 2],
}

impl Settings {
    pub fn load_settings(path: &str) -> Settings {
        let data = fs::read_to_string(path).expect("Ayar dosyası okunamadı!");

        toml::from_str::<Settings>(&data).expect("TOML parse hatası!")
    }

    pub fn save_settings(path: &str, settings: &Settings) {
        let toml_string = toml::to_string_pretty(settings).expect("TOML serialize edilemedi!");

        fs::write(path, toml_string).expect("Ayar dosyası yazılamadı!");
    }
}

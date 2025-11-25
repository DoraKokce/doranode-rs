use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::color::Color;
use raylib::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct ColorDef(pub Color);

impl<'de> Deserialize<'de> for ColorDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // "#RRGGBB" veya "#RRGGBBAA" formatını kabul edelim
        if !s.starts_with('#') {
            return Err(serde::de::Error::custom("Color must start with #"));
        }

        let hex = &s[1..];

        let (r, g, b, a) = match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(serde::de::Error::custom)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(serde::de::Error::custom)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(serde::de::Error::custom)?;
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(serde::de::Error::custom)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(serde::de::Error::custom)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(serde::de::Error::custom)?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(serde::de::Error::custom)?;
                (r, g, b, a)
            }
            _ => return Err(serde::de::Error::custom("Invalid hex color")),
        };

        Ok(ColorDef(Color { r, g, b, a }))
    }
}

#[derive(Debug, Deserialize)]
struct Scheme {
    pub colors: HashMap<String, ColorDef>,
}

impl Into<HashMap<String, Color>> for Scheme {
    fn into(self) -> HashMap<String, Color> {
        self.colors.into_iter().map(|(k, v)| (k, v.0)).collect()
    }
}

pub struct ColorSchemes {
    pub schemes: HashMap<String, HashMap<String, Color>>,
}

impl ColorSchemes {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            schemes: HashMap::new(),
        }))
    }

    pub fn load(&mut self, contents: String, scheme: String) -> &mut Self {
        let parsed: Scheme =
            serde_json::from_str(&contents).expect("Couldn't parse translation JSON");

        self.schemes.insert(scheme, parsed.into());

        self
    }

    pub fn get_color(&self, scheme: &str, color: &str) -> Option<Color> {
        let table = match self.schemes.get(scheme) {
            Some(s) => s,
            None => {
                eprintln!(
                    "Warning: Color scheme '{}' not found, using 'light'",
                    scheme
                );
                self.schemes.get("light")?
            }
        };

        match table.get(color) {
            Some(c) => Some(*c),
            None => {
                eprintln!("Warning: Color '{}' missing in scheme '{}'", color, scheme);
                None
            }
        }
    }
}

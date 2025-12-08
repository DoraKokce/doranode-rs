pub mod colorscheme;
pub mod draw;
pub mod gui;
pub mod modules;
pub mod node;
pub mod objects;
pub mod save;
pub mod settings;
pub mod structs;
pub mod translations;
pub mod window;

pub use std::{any::Any, cell::RefCell, rc::Rc, sync::Mutex};

pub use once_cell::sync::Lazy;
pub use pyo3::{IntoPyObjectExt, prelude::*};
pub use raylib::color::Color;

use crate::node::PyNode;
pub use crate::{
    colorscheme::ColorSchemes,
    objects::{
        Circle, ComboBox, Grid, Image, Object, PyObject, Rectangle, RoundedRectangle, Slider,
        TextBox, TextLabel,
    },
    settings::Settings,
    structs::Vector2,
    translations::Translations,
};

#[pymodule]
fn doranode(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyObject>();
    m.add_class::<PyNode>();
    Ok(())
}

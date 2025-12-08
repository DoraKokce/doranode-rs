use std::{cell::RefCell, env, rc::Rc};

use pyo3::{PyResult, Python, prelude::*, pymodule, types::PyModule};

use crate::{
    save::SaveFile,
    window::{EDITOR_STATE, Window},
};

mod colorscheme;
mod draw;
mod gui;
mod modules;
mod node;
mod objects;
mod save;
mod settings;
mod structs;
mod translations;
mod window;

fn main() {
    unsafe {
        let venv_path = "venv";
        env::set_var("PYO3_PYTHON", format!("{}/bin/python", venv_path));
    };
    let mut window = Window::new();
    let (rl_handle, rl_thread) = window.init();

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        EDITOR_STATE.with_borrow_mut(|state| {
            window.load_from_save(SaveFile::from_file(&args[1]).unwrap(), state);
        })
    }

    window.run(rl_handle, rl_thread);
}

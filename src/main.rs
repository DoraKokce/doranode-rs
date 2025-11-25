use crate::window::Window;

mod colorscheme;
mod node;
mod node_gui;
mod node_libary;
mod objects;
mod settings;
mod structs;
mod translations;
mod window;

fn main() {
    let mut window = Window::new();
    let (rl_handle, rl_thread) = window.init();
    window.run(rl_handle, rl_thread);
}

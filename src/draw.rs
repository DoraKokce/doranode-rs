use std::{cell::RefCell, rc::Rc};

use pyo3::{pyclass, pymethods};
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
};

use crate::structs::Vector2;

#[pyclass(unsendable)]
pub struct PyDrawHandle {
    pub draw_handle: *mut RaylibDrawHandle<'static>,
    pub font: Rc<RefCell<Font>>,
}

fn hex_to_color(color_hex: u32) -> Color {
    let r = ((color_hex >> 24) & 0xFF) as u8;
    let g = ((color_hex >> 16) & 0xFF) as u8;
    let b = ((color_hex >> 8) & 0xFF) as u8;
    let a = (color_hex & 0xFF) as u8;
    Color::new(r, g, b, a)
}

#[pymethods]
impl PyDrawHandle {
    fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: f32, color_hex: u32) {
        let font_ref = self.font.borrow();
        let handle = unsafe { &mut *self.draw_handle };

        handle.draw_text_ex(
            &*font_ref,
            text,
            Vector2::new(x as f32, y as f32, None),
            font_size,
            1.0,
            hex_to_color(color_hex),
        );
    }

    fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color_hex: u32) {
        let handle = unsafe { &mut *self.draw_handle };

        handle.draw_rectangle(x, y, width, height, hex_to_color(color_hex));
    }

    fn draw_circle(&mut self, x: i32, y: i32, radius: f32, color_hex: u32) {
        let handle = unsafe { &mut *self.draw_handle };

        handle.draw_circle(x, y, radius, hex_to_color(color_hex));
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color_hex: u32) {
        let handle = unsafe { &mut *self.draw_handle };

        handle.draw_line(x1, y1, x2, y2, hex_to_color(color_hex));
    }
}

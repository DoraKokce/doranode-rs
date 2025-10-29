use std::{cell::RefCell, fmt::Debug, rc::Rc};

use raylib::prelude::*;
use raylib_sys::CheckCollisionPointRec;

use crate::structs::Vector2;

pub trait Object {
    fn z_index(&self) -> i32;
    fn position(&self) -> Vector2;
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera);
    fn update(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread, _camera: &Camera) {}
    fn set_position(&mut self, position: Vector2);
}

/* RECTANGLE */
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub position: Vector2,
    pub size: Vector2,
    pub background_color: Color,
    pub border_thickness: Option<u32>,
    pub border_color: Option<Color>,
    pub z: i32,
}

impl Object for Rectangle {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.background_color,
        );

        if let (Some(thickness), Some(border_color)) = (self.border_thickness, self.border_color) {
            let rect = raylib::prelude::Rectangle::from(self.clone());
            draw_handle.draw_rectangle_lines_ex(rect, thickness as f32, border_color);
        }
    }
}

impl From<&Rectangle> for raylib::prelude::Rectangle {
    fn from(value: &Rectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x as f32,
            value.position.y as f32,
            value.size.x as f32,
            value.size.y as f32,
        )
    }
}

impl From<Rectangle> for raylib::prelude::Rectangle {
    fn from(value: Rectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x as f32,
            value.position.y as f32,
            value.size.x as f32,
            value.size.y as f32,
        )
    }
}

/* ROUNDED RECTANGLE */
#[derive(Debug, Clone)]
pub struct RoundedRectangle {
    pub position: Vector2,
    pub size: Vector2,
    pub roundness: f32,
    pub background_color: Color,
    pub border_thickness: Option<u32>,
    pub border_color: Option<Color>,
    pub z: i32,
}

impl Object for RoundedRectangle {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let rect = raylib::prelude::Rectangle::from(self.clone());

        draw_handle.draw_rectangle_rounded(rect, self.roundness as f32, 32, self.background_color);

        if let (Some(thickness), Some(border_color)) = (self.border_thickness, self.border_color) {
            draw_handle.draw_rectangle_rounded_lines_ex(
                rect,
                self.roundness,
                32,
                thickness as f32,
                border_color,
            );
        }
    }
}

impl From<&RoundedRectangle> for raylib::prelude::Rectangle {
    fn from(value: &RoundedRectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x as f32,
            value.position.y as f32,
            value.size.x as f32,
            value.size.y as f32,
        )
    }
}

impl From<RoundedRectangle> for raylib::prelude::Rectangle {
    fn from(value: RoundedRectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x as f32,
            value.position.y as f32,
            value.size.x as f32,
            value.size.y as f32,
        )
    }
}

/* CIRCLE */
#[derive(Debug, Clone)]
pub struct Circle {
    pub position: Vector2,
    pub radius: f32,
    pub background_color: Color,
    pub border_thickness: Option<u32>,
    pub border_color: Option<Color>,
    pub z: i32,
}

impl Object for Circle {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_circle(
            self.position.x,
            self.position.y,
            self.radius,
            self.background_color,
        );

        if let (Some(thickness), Some(border_color)) = (self.border_thickness, self.border_color) {
            for i in 0..thickness {
                draw_handle.draw_circle_lines(
                    self.position.x,
                    self.position.y,
                    self.radius + i as f32,
                    border_color,
                );
            }
        }
    }
}

/* GRID */
#[derive(Debug, Clone)]
pub struct Grid {
    pub position: Vector2,
    pub size: Vector2,
    pub square_size: Vector2,
    pub square_color: Color,
    pub background_color: Color,
    pub big_square_size: Option<Vector2>,
    pub big_square_color: Option<Color>,
    pub z: i32,
}

impl Object for Grid {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let max_x = self.size.x * self.square_size.x;
        let max_y = self.size.y * self.square_size.y;

        draw_handle.draw_rectangle(
            self.position.x,
            self.position.y,
            max_x,
            max_y,
            self.background_color,
        );

        let pick_color = |index: i32, axis_size: Option<Vector2>, axis_color: Option<Color>| match (
            axis_size, axis_color,
        ) {
            (Some(size), Some(color)) if index % size.x == 0 => color,
            _ => self.square_color,
        };

        for x in 0..=self.size.x {
            let color = pick_color(x, self.big_square_size.clone(), self.big_square_color);
            let x_pos = self.position.x + x * self.square_size.x;
            draw_handle.draw_line(
                x_pos,
                self.position.y,
                x_pos,
                self.position.y + max_y,
                color,
            );
        }

        for y in 0..=self.size.y {
            let color = pick_color(y, self.big_square_size.clone(), self.big_square_color);
            let y_pos = self.position.y + y * self.square_size.y;
            draw_handle.draw_line(
                self.position.x,
                y_pos,
                self.position.x + max_x,
                y_pos,
                color,
            );
        }
    }
}

/* TEXT */
pub struct TextLabel {
    pub position: Vector2,
    pub foreground_color: Color,
    pub font: Rc<Font>,
    pub font_size: f32,
    pub text: String,
    pub z: i32,
}

impl Object for TextLabel {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let rl_vec: raylib::prelude::Vector2 = raylib::prelude::Vector2::from(&self.position);

        draw_handle.draw_text_ex(
            &*self.font,
            &self.text,
            rl_vec,
            self.font_size,
            1.0,
            self.foreground_color,
        );
    }
}

/* IMAGE */
#[derive(Debug)]
pub struct Image {
    pub position: Vector2,
    pub texture: Option<Texture2D>,
    pub size: Vector2,
    pub z: i32,
}

impl Image {
    pub fn get_image_from_path(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: String,
    ) {
        if let Ok(tex) = rl.load_texture(thread, &path) {
            self.texture = Some(tex);
        } else {
            eprintln!("[-] Resim {} yüklenemedi", path);
        }
    }
}

impl Object for Image {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        if let Some(tex) = &self.texture {
            draw_handle.draw_texture_rec(tex, self, self.position(), Color::WHITE);
        }
    }
}

impl From<&Image> for raylib_sys::Rectangle {
    fn from(value: &Image) -> raylib_sys::Rectangle {
        raylib_sys::Rectangle {
            x: value.position.x as f32,
            y: value.position.y as f32,
            width: value.size.x as f32,
            height: value.size.y as f32,
        }
    }
}

/* TEXTBOX */
#[derive(Debug)]
pub struct TextBox {
    pub position: Vector2,
    pub size: Vector2,
    pub background_color: Color,
    pub active_background_color: Color,
    pub foreground_color: Color,
    pub border_color: Option<Color>,
    pub border_thickness: Option<u32>,
    pub active: bool,
    pub text: String,
    pub cursor_index: usize,
    pub font_size: i32,
    pub font: Font,
    pub scroll_offset: usize,
    pub cursor_blink: bool,
    pub is_editable: bool,
    pub scalable: bool,
    pub min_size: Option<Vector2>,
    pub z: i32,
}

impl Object for TextBox {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera) {
        let rect: Rectangle = self.into();
        rect.draw(draw_handle, camera);
        let screen_position = draw_handle
            .get_world_to_screen2D(self.position.clone(), Camera2D::from(camera.clone()));

        let scroll_prefix: String = self.text.chars().take(self.scroll_offset).collect();
        let scroll_px = self
            .font
            .measure_text(&scroll_prefix, self.font_size as f32, 1.0)
            .x as i32;

        draw_handle.draw_scissor_mode(
            screen_position.x as i32 + 8,
            screen_position.y as i32 + 4,
            self.size.x - 16,
            self.size.y - 8,
            |mut scissor| {
                let draw_pos = self.position.clone() + Vector2::new(8 - scroll_px, 4, None);
                scissor.draw_text_ex(
                    &self.font,
                    &self.text,
                    draw_pos,
                    self.font_size as f32,
                    1.0,
                    self.foreground_color,
                );

                if self.cursor_blink {
                    let cursor_full: String = self.text.chars().take(self.cursor_index).collect();
                    let cursor_full_px = self
                        .font
                        .measure_text(&cursor_full, self.font_size as f32, 1.0)
                        .x as i32;

                    let cursor_pos_x = cursor_full_px - scroll_px;

                    scissor.draw_rectangle(
                        self.position.x + 8 + cursor_pos_x,
                        self.position.y + 4,
                        2,
                        self.font_size,
                        self.foreground_color,
                    );
                }
            },
        );
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, camera: &Camera) {
        self.font_size = self.size.y - 8;
        self.cursor_blink =
            (rl.get_time() * 2.0) as i32 % 2 == 0 && self.active && self.is_editable;
        if self.is_editable == false {
            return;
        }
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), Camera2D::from(camera));
        let rect: Rectangle = self.into();

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.active = unsafe {
                CheckCollisionPointRec(
                    ffi::Vector2::from(mouse_pos) as raylib_sys::Vector2,
                    (prelude::Rectangle::from(rect) as raylib::prelude::Rectangle).into(),
                )
            };
        }

        if !self.active {
            self.cursor_index = 0;
            self.scroll_offset = 0;
            return;
        }

        while let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_BACKSPACE => {
                    if self.cursor_index > 0 {
                        let (byte_index, _) =
                            self.text.char_indices().nth(self.cursor_index - 1).unwrap();

                        self.text.remove(byte_index);

                        self.cursor_index -= 1;
                    }
                }
                KeyboardKey::KEY_DELETE => {
                    if self.cursor_index < self.text.chars().count() {
                        let (byte_index, _) =
                            self.text.char_indices().nth(self.cursor_index).unwrap();

                        self.text.remove(byte_index);
                    }
                }
                KeyboardKey::KEY_LEFT => {
                    if self.cursor_index > 0 {
                        self.cursor_index -= 1;
                    }
                }
                KeyboardKey::KEY_RIGHT => {
                    if self.cursor_index < self.text.chars().count() {
                        self.cursor_index += 1;
                    }
                }
                KeyboardKey::KEY_ENTER | KeyboardKey::KEY_ESCAPE => {
                    self.active = false;
                }
                _ => {}
            }
        }

        while let Some(ch) = rl.get_char_pressed() {
            if ch != 0 as char {
                if let Some(c) = std::char::from_u32(ch as u32) {
                    let byte_index = self
                        .text
                        .char_indices()
                        .nth(self.cursor_index)
                        .map(|(i, _)| i)
                        .unwrap_or(self.text.len());
                    self.text.insert(byte_index, c);
                    self.cursor_index += 1;
                }
            }
        }

        let total_chars = self.text.chars().count();
        if self.scroll_offset > total_chars {
            self.scroll_offset = total_chars;
        }
        let visible_px = self.size.x - 16;

        let scroll_prefix: String = self.text.chars().take(self.scroll_offset).collect();
        let mut scroll_px = self
            .font
            .measure_text(&scroll_prefix, self.font_size as f32, 1.0)
            .x as i32;

        if self.scalable {
            let text_px = self
                .font
                .measure_text(&self.text, self.font_size as f32, 1.0)
                .x as i32;
            if text_px + 16 > self.size.x {
                self.size.x = text_px;
            } else if let Some(min_size) = &self.min_size {
                if self.size.x < min_size.x {
                    self.size.x = min_size.x;
                }
            }
            return;
        }

        let cursor_prefix: String = self.text.chars().take(self.cursor_index).collect();
        let cursor_px = self
            .font
            .measure_text(&cursor_prefix, self.font_size as f32, 1.0)
            .x as i32;
        if cursor_px - scroll_px > visible_px - 4 {
            while self.scroll_offset < total_chars {
                self.scroll_offset += 1;
                let new_prefix: String = self.text.chars().take(self.scroll_offset).collect();
                scroll_px = self
                    .font
                    .measure_text(&new_prefix, self.font_size as f32, 1.0)
                    .x as i32;
                if cursor_px - scroll_px <= visible_px - 4 {
                    break;
                }
            }
        }

        while self.scroll_offset > 0 {
            let prefix: String = self.text.chars().take(self.scroll_offset).collect();
            scroll_px = self
                .font
                .measure_text(&prefix, self.font_size as f32, 1.0)
                .x as i32;
            if cursor_px >= scroll_px + 4 {
                break;
            }
            self.scroll_offset = self.scroll_offset.saturating_sub(1);
        }
    }
}

impl From<&mut TextBox> for Rectangle {
    fn from(value: &mut TextBox) -> Self {
        Self {
            background_color: if value.active {
                value.active_background_color
            } else {
                value.background_color
            },
            border_color: value.border_color,
            border_thickness: value.border_thickness,
            position: value.position.clone(),
            size: value.size.clone(),
            z: value.z,
        }
    }
}

impl From<&TextBox> for Rectangle {
    fn from(value: &TextBox) -> Self {
        Self {
            background_color: if value.active {
                value.active_background_color
            } else {
                value.background_color
            },
            border_color: value.border_color,
            border_thickness: value.border_thickness,
            position: value.position.clone(),
            size: value.size.clone(),
            z: value.z,
        }
    }
}

/* SLIDER */
#[derive(Debug)]
pub struct Slider<T: AsF32 + From<f32>> {
    pub position: Vector2,
    pub size: Vector2,
    pub min_value: T,
    pub max_value: T,
    pub current_value: T,
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
    pub handle_color: Color,
    pub step: Option<T>,
    pub z: i32,
}

impl<T: AsF32 + From<f32>> Object for Slider<T> {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        if let Some(bg_color) = self.background_color {
            draw_handle.draw_rectangle(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
                bg_color,
            );
            draw_handle.draw_circle(
                self.position.x,
                self.position.y + self.size.y / 2,
                self.size.y as f32 / 2.0,
                bg_color,
            );
            draw_handle.draw_circle(
                self.position.x + self.size.x,
                self.position.y + self.size.y / 2,
                self.size.y as f32 / 2.0,
                bg_color,
            );
        }

        let filled_width = ((self.current_value.as_f32() - self.min_value.as_f32())
            / (self.max_value.as_f32() - self.min_value.as_f32()))
            * self.size.x as f32;

        if let Some(fg_color) = self.foreground_color {
            draw_handle.draw_rectangle(
                self.position.x,
                self.position.y,
                filled_width as i32,
                self.size.y,
                fg_color,
            );
            if filled_width > 0.0 {
                draw_handle.draw_circle(
                    self.position.x,
                    self.position.y + self.size.y / 2,
                    self.size.y as f32 / 2.0,
                    fg_color,
                );
            }
        }

        let handle_x = self.position.x + filled_width as i32;
        draw_handle.draw_circle(
            handle_x,
            self.position.y + self.size.y / 2,
            10.0,
            self.handle_color,
        );
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, camera: &Camera) {
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), Camera2D::from(camera));

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let px = mouse_pos.x;
            let py = mouse_pos.y;

            let x0 = self.position.x as f32;
            let y0 = self.position.y as f32;
            let x1 = x0 + self.size.x as f32;
            let y1 = y0 + self.size.y as f32;

            if px >= x0 && px <= x1 && py >= y0 && py <= y1 {
                let relative = (px - x0).clamp(0.0, self.size.x as f32);
                let ratio = relative / (self.size.x as f32).max(f32::EPSILON);
                let mut val = self.min_value.as_f32()
                    + ratio * (self.max_value.as_f32() - self.min_value.as_f32());

                if val < self.min_value.as_f32() {
                    val = self.min_value.as_f32();
                } else if val > self.max_value.as_f32() {
                    val = self.max_value.as_f32();
                }

                let step = self.step.unwrap_or(0.05.into()).as_f32();
                let val = (val / step).round() * step;

                self.current_value = val.into();
            }
        }
    }
}

/* CAMERA */
#[derive(Debug, Clone)]
pub struct Camera {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

impl From<Camera> for prelude::Camera2D {
    fn from(value: Camera) -> prelude::Camera2D {
        prelude::Camera2D {
            offset: value.offset.into(),
            target: value.target.into(),
            rotation: value.rotation,
            zoom: value.zoom,
        }
    }
}

impl From<&Camera> for prelude::Camera2D {
    fn from(value: &Camera) -> Self {
        Self {
            offset: value.offset.clone().into(),
            target: value.target.clone().into(),
            rotation: value.rotation,
            zoom: value.zoom,
        }
    }
}

impl From<Camera> for raylib_sys::Camera2D {
    fn from(value: Camera) -> Self {
        Self {
            offset: value.offset.into(),
            target: value.target.into(),
            rotation: value.rotation,
            zoom: value.zoom,
        }
    }
}

impl From<&Camera> for raylib_sys::Camera2D {
    fn from(value: &Camera) -> Self {
        Self {
            offset: value.offset.clone().into(),
            target: value.target.clone().into(),
            rotation: value.rotation,
            zoom: value.zoom,
        }
    }
}

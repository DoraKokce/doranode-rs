use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use pyo3::{IntoPyObjectExt, prelude::*};
use raylib::prelude::*;
use raylib_sys::CheckCollisionPointRec;

use crate::structs::Vector2;

pub trait Object {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera);
    fn update(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread, _camera: &Camera) {}
    fn set_property(&mut self, key: String, value: Box<dyn Any>);
    fn get_property(&self, key: String) -> Box<dyn Any + 'static>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/* RECTANGLE */
pub struct Rectangle {
    pub position: Vector2,
    pub size: Vector2,
    pub background_color: Color,
    pub border_color: Option<Color>,
    pub border_thickness: Option<f32>,
    pub z: i32,
}

impl Object for Rectangle {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_rectangle_pro(
            raylib::prelude::Rectangle::new(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
            ),
            Vector2::zero(),
            0.0,
            self.background_color,
        );

        if let (Some(thickness), Some(border_color)) = (self.border_thickness, self.border_color) {
            let rect = raylib::prelude::Rectangle {
                x: self.position.x,
                y: self.position.y,
                width: self.size.x,
                height: self.size.y,
            };
            draw_handle.draw_rectangle_lines_ex(rect, thickness, border_color);
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = *v;
                }
            }
            "border_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.border_color = Some(*v);
                }
            }
            "border_thickness" => {
                if let Ok(v) = value.downcast::<f32>() {
                    self.border_thickness = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "background_color" => Box::new(self.background_color),
            "border_color" => Box::new(self.border_color),
            "border_thickness" => Box::new(self.border_thickness),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl From<&Rectangle> for raylib::prelude::Rectangle {
    fn from(value: &Rectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x,
            value.position.y,
            value.size.x,
            value.size.y,
        )
    }
}

impl From<Rectangle> for raylib::prelude::Rectangle {
    fn from(value: Rectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x,
            value.position.y,
            value.size.x,
            value.size.y,
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
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let rect = raylib::prelude::Rectangle::from(self.clone());

        draw_handle.draw_rectangle_rounded(rect, self.roundness, 32, self.background_color);

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

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "roundness" => {
                if let Ok(v) = value.downcast::<f32>() {
                    self.roundness = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = *v;
                }
            }
            "border_thickness" => {
                if let Ok(v) = value.downcast::<u32>() {
                    self.border_thickness = Some(*v);
                }
            }
            "border_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.border_color = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "roundness" => Box::new(self.roundness),
            "background_color" => Box::new(self.background_color),
            "border_thickness" => Box::new(self.border_thickness),
            "border_color" => Box::new(self.border_color),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl From<&RoundedRectangle> for raylib::prelude::Rectangle {
    fn from(value: &RoundedRectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x,
            value.position.y,
            value.size.x,
            value.size.y,
        )
    }
}

impl From<RoundedRectangle> for raylib::prelude::Rectangle {
    fn from(value: RoundedRectangle) -> raylib::prelude::Rectangle {
        raylib::prelude::Rectangle::new(
            value.position.x,
            value.position.y,
            value.size.x,
            value.size.y,
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
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            self.radius,
            self.background_color,
        );

        if let (Some(thickness), Some(border_color)) = (self.border_thickness, self.border_color) {
            for i in 0..thickness {
                draw_handle.draw_circle_lines(
                    self.position.x as i32,
                    self.position.y as i32,
                    self.radius + i as f32,
                    border_color,
                );
            }
        }
    }
    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "radius" => {
                if let Ok(v) = value.downcast::<f32>() {
                    self.radius = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = *v;
                }
            }
            "border_thickness" => {
                if let Ok(v) = value.downcast::<u32>() {
                    self.border_thickness = Some(*v);
                }
            }
            "border_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.border_color = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "radius" => Box::new(self.radius),
            "background_color" => Box::new(self.background_color),
            "border_thickness" => Box::new(self.border_thickness),
            "border_color" => Box::new(self.border_color),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let max_x = self.size.x * self.square_size.x;
        let max_y = self.size.y * self.square_size.y;

        draw_handle.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            max_x as i32,
            max_y as i32,
            self.background_color,
        );

        let pick_color = |index: i32, axis_size: Option<Vector2>, axis_color: Option<Color>| match (
            axis_size, axis_color,
        ) {
            (Some(size), Some(color)) if index % size.x as i32 == 0 => color,
            _ => self.square_color,
        };

        for x in 0..=self.size.x as i32 {
            let color = pick_color(x, self.big_square_size.clone(), self.big_square_color);
            let x_pos = self.position.x as i32 + x * self.square_size.x as i32;
            draw_handle.draw_line(
                x_pos,
                self.position.y as i32,
                x_pos,
                (self.position.y + max_y) as i32,
                color,
            );
        }

        for y in 0..=self.size.y as i32 {
            let color = pick_color(y, self.big_square_size.clone(), self.big_square_color);
            let y_pos = self.position.y as i32 + y * self.square_size.y as i32;
            draw_handle.draw_line(
                self.position.x as i32,
                y_pos,
                (self.position.x + max_x) as i32,
                y_pos,
                color,
            );
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "square_size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.square_size = *v;
                }
            }
            "square_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.square_color = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = *v;
                }
            }
            "big_square_size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.big_square_size = Some(*v);
                }
            }
            "big_square_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.big_square_color = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key for grid: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "square_size" => Box::new(self.square_size.clone()),
            "square_color" => Box::new(self.square_color),
            "background_color" => Box::new(self.background_color),
            "big_square_size" => Box::new(self.big_square_size.clone()),
            "big_square_color" => Box::new(self.big_square_color),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key for grid: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/* COMBOBOX */

pub struct ComboBox {
    pub position: Vector2,
    pub size: Vector2,
    pub options: Vec<String>,
    pub background_color: Color,
    pub active_background_color: Color,
    pub foreground_color: Color,
    pub border_color: Option<Color>,
    pub border_thickness: Option<i32>,
    pub font: Rc<RefCell<Font>>,
    pub font_size: f32,
    selected: usize,
    open: bool,
    z: i32,
}

impl ComboBox {
    pub fn new(
        position: Vector2,
        size: Vector2,
        options: Vec<String>,
        background_color: Color,
        active_background_color: Color,
        foreground_color: Color,
        border_color: Option<Color>,
        border_thickness: Option<i32>,
        font: Rc<RefCell<Font>>,
        font_size: f32,
        z: i32,
    ) -> Self {
        Self {
            position,
            size,
            options,
            background_color,
            active_background_color,
            foreground_color,
            border_color,
            border_thickness,
            font,
            font_size,
            selected: 0,
            open: false,
            z,
        }
    }
}

impl Object for ComboBox {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            self.size.x as i32,
            self.size.y as i32,
            if self.open {
                self.active_background_color
            } else {
                self.background_color
            },
        );

        if let (Some(border_color), Some(border_thickness)) =
            (self.border_color, self.border_thickness)
        {
            draw_handle.draw_rectangle_lines_ex(
                raylib::prelude::Rectangle::new(
                    self.position.x,
                    self.position.y,
                    self.size.x,
                    self.size.y,
                ),
                border_thickness as f32,
                border_color,
            );
        }

        draw_handle.draw_text_ex(
            &*self.font.borrow(),
            &self.options[self.selected],
            self.position.clone() + Vector2::new(5.0, (self.size.y - self.font_size) / 4.0, None),
            self.font_size,
            1.0,
            self.foreground_color,
        );

        draw_handle.draw_text_ex(
            &*self.font.borrow(),
            "v",
            self.position.clone()
                + Vector2::new(
                    self.size.x - (self.size.x - self.font_size) / 4.0,
                    (self.size.y - self.font_size) / 4.0,
                    None,
                ),
            self.font_size,
            1.0,
            self.foreground_color,
        );

        if self.open {
            for (i, opt) in self.options.iter().enumerate() {
                let oy = self.position.y + self.size.y * (i as f32 + 1.0);

                draw_handle.draw_rectangle(
                    self.position.x as i32,
                    oy as i32,
                    self.size.x as i32,
                    self.size.y as i32,
                    self.active_background_color,
                );
                draw_handle.draw_text_ex(
                    &*self.font.borrow(),
                    opt,
                    Vector2::new(
                        self.position.x + 5.0,
                        oy + (self.size.y - self.font_size) / 4.0,
                        None,
                    ),
                    self.font_size,
                    1.0,
                    self.foreground_color,
                );
            }
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, camera: &Camera) {
        let mouse = rl.get_screen_to_world2D(rl.get_mouse_position(), camera);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if mouse.x >= self.position.x
                && mouse.x <= (self.position.x + self.size.x)
                && mouse.y >= self.position.y
                && mouse.y <= (self.position.y + self.size.y)
            {
                self.open = !self.open;
            } else if self.open {
                for (i, _) in self.options.iter().enumerate() {
                    let oy = self.position.y + self.size.y * (i as f32 + 1.0);

                    if mouse.x >= self.position.x
                        && mouse.x <= (self.position.x + self.size.x)
                        && mouse.y >= oy
                        && mouse.y <= (oy + self.size.y)
                    {
                        self.selected = i;
                        self.open = false;
                    }
                }
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "z" => {
                if let Ok(z) = value.downcast::<i32>() {
                    self.z = *z
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }
}

/* TEXT */
pub struct TextLabel {
    pub position: Vector2,
    pub foreground_color: Color,
    pub font: Rc<RefCell<Font>>,
    pub font_size: f32,
    pub text: String,
    pub z: i32,
}

impl Object for TextLabel {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        let rl_vec: raylib::prelude::Vector2 = raylib::prelude::Vector2::from(&self.position);

        draw_handle.draw_text_ex(
            &*self.font.borrow(),
            &self.text,
            rl_vec,
            self.font_size,
            1.0,
            self.foreground_color,
        );
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "foreground_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.foreground_color = *v;
                }
            }
            "font" => {
                if let Ok(v) = value.downcast::<Rc<RefCell<Font>>>() {
                    self.font = *v.clone();
                }
            }
            "font_size" => {
                if let Ok(v) = value.downcast::<f32>() {
                    self.font_size = *v;
                }
            }
            "text" => {
                if let Ok(v) = value.downcast::<String>() {
                    self.text = (*v).clone();
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "foreground_color" => Box::new(self.foreground_color),
            "font" => Box::new(self.font.clone()),
            "font_size" => Box::new(self.font_size),
            "text" => Box::new(self.text.clone()),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/* IMAGE */
#[derive(Debug)]
pub struct Image {
    pub position: Vector2,
    pub texture: Option<Rc<Texture2D>>,
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
        match rl.load_texture(thread, &path) {
            Ok(tex) => self.texture = Some(Rc::new(tex)),
            Err(_) => eprintln!("[-] Resim {} yüklenemedi", path),
        }
    }
}

impl Object for Image {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        if let Some(tex) = &self.texture {
            draw_handle.draw_texture_rec(&**tex, self, self.position.clone(), Color::WHITE);
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any + 'static>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "texture" => {
                if let Ok(v) = value.downcast::<Texture2D>() {
                    self.texture = Some(Rc::new(*v));
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => {
                eprintln!("Unknown property key: {}", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "texture" => Box::new(self.texture.clone()),
            "size" => Box::new(self.size.clone()),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl From<&Image> for raylib_sys::Rectangle {
    fn from(value: &Image) -> raylib_sys::Rectangle {
        raylib_sys::Rectangle {
            x: value.position.x,
            y: value.position.y,
            width: value.size.x,
            height: value.size.y,
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
    active: bool,
    pub text: String,
    cursor_index: usize,
    pub font_size: i32,
    pub font: Rc<RefCell<Font>>,
    scroll_offset: usize,
    cursor_blink: bool,
    allowed_chars: Option<Vec<char>>,
    pub is_editable: bool,
    pub scalable: bool,
    pub min_size: Option<Vector2>,
    pub z: i32,
}

impl Object for TextBox {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera) {
        let rect = Rectangle::from(self);
        rect.draw(draw_handle, camera);
        let screen_position = draw_handle
            .get_world_to_screen2D(self.position.clone(), Camera2D::from(camera.clone()));

        let scroll_prefix: String = self.text.chars().take(self.scroll_offset).collect();
        let scroll_px = self
            .font
            .borrow()
            .measure_text(&scroll_prefix, self.font_size as f32, 1.0)
            .x as i32;

        draw_handle.draw_scissor_mode(
            screen_position.x as i32 + 8,
            screen_position.y as i32 + 4,
            ((self.size.x - 16.0) * camera.zoom) as i32,
            ((self.size.y - 8.0) * camera.zoom) as i32,
            |mut scissor| {
                let draw_pos =
                    self.position.clone() + Vector2::new(8.0 - scroll_px as f32, 0.0, None);
                scissor.draw_text_ex(
                    &*self.font.borrow(),
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
                        .borrow()
                        .measure_text(&cursor_full, self.font_size as f32, 1.0)
                        .x as i32;

                    let cursor_pos_x = cursor_full_px - scroll_px;

                    scissor.draw_rectangle(
                        self.position.x as i32 + 8 + cursor_pos_x,
                        self.position.y as i32,
                        2,
                        self.font_size,
                        self.foreground_color,
                    );
                }
            },
        );
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, camera: &Camera) {
        self.font_size = self.size.y as i32 - 8;
        self.cursor_blink =
            (rl.get_time() * 2.0) as i32 % 2 == 0 && self.active && self.is_editable;

        if self.scalable
            && let Some(min_size) = &self.min_size
        {
            let text_px = self
                .font
                .borrow()
                .measure_text(&self.text, self.font_size as f32, 1.0)
                .x;
            self.size.x = min_size.x.max(text_px + 16.0);
            self.size.y = min_size.y.max(self.size.y);
        }

        if self.is_editable == false {
            return;
        }
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), Camera2D::from(camera));
        let rect: Rectangle = self.into();

        let is_hover = raylib::prelude::Rectangle::from(rect).check_collision_point_rec(mouse_pos);

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.active = is_hover
        }

        if !self.active {
            self.cursor_index = 0;
            self.scroll_offset = 0;
            return;
        }

        while let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_BACKSPACE => {
                    if self.cursor_index > 0 && self.cursor_index < self.text.chars().count() + 1 {
                        let mut char_indices = self.text.char_indices();
                        let (byte_index, _) = char_indices
                            .nth(self.cursor_index - 1)
                            .expect("cursor_index geçersiz");
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
                    if let Some(allowed) = self.allowed_chars.take() {
                        if allowed.contains(&c) {
                            self.text.insert(byte_index, c);
                            self.cursor_index += 1;
                        }
                        self.allowed_chars = Some(allowed);
                    } else {
                        self.text.insert(byte_index, c);
                        self.cursor_index += 1;
                    }
                }
            }
        }

        if self.scalable {
            return;
        }

        let total_chars = self.text.chars().count();
        if self.scroll_offset > total_chars {
            self.scroll_offset = total_chars;
        }
        let visible_px = self.size.x as i32 - 16;

        let scroll_prefix: String = self.text.chars().take(self.scroll_offset).collect();
        let mut scroll_px = self
            .font
            .borrow()
            .measure_text(&scroll_prefix, self.font_size as f32, 1.0)
            .x as i32;

        let cursor_prefix: String = self.text.chars().take(self.cursor_index).collect();
        let cursor_px = self
            .font
            .borrow()
            .measure_text(&cursor_prefix, self.font_size as f32, 1.0)
            .x as i32;
        if cursor_px - scroll_px > visible_px - 4 {
            while self.scroll_offset < total_chars {
                self.scroll_offset += 1;
                let new_prefix: String = self.text.chars().take(self.scroll_offset).collect();
                scroll_px = self
                    .font
                    .borrow()
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
                .borrow()
                .measure_text(&prefix, self.font_size as f32, 1.0)
                .x as i32;
            if cursor_px >= scroll_px + 4 {
                break;
            }
            self.scroll_offset = self.scroll_offset.saturating_sub(1);
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = *v;
                }
            }
            "active_background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.active_background_color = *v;
                }
            }
            "foreground_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.foreground_color = *v;
                }
            }
            "border_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.border_color = Some(*v);
                }
            }
            "border_thickness" => {
                if let Ok(v) = value.downcast::<u32>() {
                    self.border_thickness = Some(*v);
                }
            }
            "text" => {
                if let Ok(v) = value.downcast::<String>() {
                    self.text = *v;
                }
            }
            "font_size" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.font_size = *v;
                }
            }
            "is_editable" => {
                if let Ok(v) = value.downcast::<bool>() {
                    self.is_editable = *v;
                }
            }
            "scalable" => {
                if let Ok(v) = value.downcast::<bool>() {
                    self.scalable = *v;
                }
            }
            "min_size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.min_size = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => eprintln!("Unknown property key: {}", key),
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "background_color" => Box::new(self.background_color),
            "active_background_color" => Box::new(self.active_background_color),
            "foreground_color" => Box::new(self.foreground_color),
            "border_color" => Box::new(self.border_color),
            "border_thickness" => Box::new(self.border_thickness),
            "text" => Box::new(self.text.clone()),
            "font_size" => Box::new(self.font_size),
            "font" => Box::new(self.font.clone()),
            "is_editable" => Box::new(self.is_editable),
            "scalable" => Box::new(self.scalable),
            "min_size" => Box::new(self.min_size.clone()),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl TextBox {
    pub fn new(
        position: Vector2,
        size: Vector2,
        background_color: Color,
        active_background_color: Color,
        foreground_color: Color,
        border_color: Option<Color>,
        border_thickness: Option<u32>,
        font: Rc<RefCell<Font>>,
        is_editable: bool,
        scalable: bool,
        min_size: Option<Vector2>,
        allowed_chars: Option<Vec<char>>,
        z: i32,
    ) -> Self {
        Self {
            position,
            size,
            background_color,
            active_background_color,
            foreground_color,
            border_color,
            border_thickness,
            active: false,
            text: String::new(),
            cursor_index: 0,
            font_size: 0,
            font,
            scroll_offset: 0,
            cursor_blink: false,
            is_editable,
            scalable,
            min_size,
            allowed_chars,
            z,
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
            border_thickness: value.border_thickness.map(|x| x as f32),
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
            border_thickness: value.border_thickness.map(|x| x as f32),
            position: value.position.clone(),
            size: value.size.clone(),
            z: value.z,
        }
    }
}

/* SLIDER */
#[derive(Debug)]
pub struct Slider<T: AsF32 + From<f32> + 'static> {
    pub position: Vector2,
    pub size: Vector2,
    pub min_value: T,
    pub max_value: T,
    pub value: T,
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
    pub handle_color: Color,
    pub step: Option<T>,
    pub z: i32,
}

impl<T: AsF32 + From<f32> + 'static> Object for Slider<T> {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        if let Some(bg_color) = self.background_color {
            draw_handle.draw_rectangle(
                self.position.x as i32,
                self.position.y as i32,
                self.size.x as i32,
                self.size.y as i32,
                bg_color,
            );
            draw_handle.draw_circle(
                self.position.x as i32,
                self.position.y as i32 + self.size.y as i32 / 2,
                self.size.y / 2.0,
                bg_color,
            );
            draw_handle.draw_circle(
                self.position.x as i32 + self.size.x as i32,
                self.position.y as i32 + self.size.y as i32 / 2,
                self.size.y / 2.0,
                bg_color,
            );
        }

        let filled_width = ((self.value.as_f32() - self.min_value.as_f32())
            / (self.max_value.as_f32() - self.min_value.as_f32()))
            * self.size.x;

        if let Some(fg_color) = self.foreground_color {
            draw_handle.draw_rectangle(
                self.position.x as i32,
                self.position.y as i32,
                filled_width as i32,
                self.size.y as i32,
                fg_color,
            );
            if filled_width > 0.0 {
                draw_handle.draw_circle(
                    self.position.x as i32,
                    self.position.y as i32 + self.size.y as i32 / 2,
                    self.size.y / 2.0,
                    fg_color,
                );
            }
        }

        let handle_x = self.position.x + filled_width;
        draw_handle.draw_circle(
            handle_x as i32,
            self.position.y as i32 + self.size.y as i32 / 2,
            10.0,
            self.handle_color,
        );
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, camera: &Camera) {
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), Camera2D::from(camera));

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let px = mouse_pos.x;
            let py = mouse_pos.y;

            let x0 = self.position.x;
            let y0 = self.position.y;
            let x1 = x0 + self.size.x;
            let y1 = y0 + self.size.y;

            if px >= x0 && px <= x1 && py >= y0 && py <= y1 {
                let relative = (px - x0).clamp(0.0, self.size.x);
                let ratio = relative / (self.size.x).max(f32::EPSILON);
                let mut val = self.min_value.as_f32()
                    + ratio * (self.max_value.as_f32() - self.min_value.as_f32());

                if val < self.min_value.as_f32() {
                    val = self.min_value.as_f32();
                } else if val > self.max_value.as_f32() {
                    val = self.max_value.as_f32();
                }

                let step = self.step.unwrap_or(0.05.into()).as_f32();
                let val = (val / step).round() * step;

                self.value = val.into();
            }
        }
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "size" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.size = *v;
                }
            }
            "min_value" => {
                if let Ok(v) = value.downcast::<T>() {
                    self.min_value = *v;
                }
            }
            "max_value" => {
                if let Ok(v) = value.downcast::<T>() {
                    self.max_value = *v;
                }
            }
            "value" => {
                if let Ok(v) = value.downcast::<T>() {
                    self.value = *v;
                }
            }
            "background_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.background_color = Some(*v);
                }
            }
            "foreground_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.foreground_color = Some(*v);
                }
            }
            "handle_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.handle_color = *v;
                }
            }
            "step" => {
                if let Ok(v) = value.downcast::<T>() {
                    self.step = Some(*v);
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => eprintln!("Unknown property key for set: {}", key),
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "min_value" => Box::new(self.min_value.clone()),
            "max_value" => Box::new(self.max_value.clone()),
            "value" => Box::new(self.value.clone()),
            "background_color" => Box::new(self.background_color),
            "foreground_color" => Box::new(self.foreground_color),
            "handle_color" => Box::new(self.handle_color),
            "step" => Box::new(self.step.clone()),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("Unknown property key for get: {}", key);
                Box::new(())
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

#[pyclass(unsendable)]
pub struct PyObjectWrapper {
    inner: Rc<RefCell<Box<dyn Object>>>,
}

#[pymethods]
impl PyObjectWrapper {
    pub fn get_property(&self, name: String, py: Python) -> PyResult<Py<PyAny>> {
        let prop = self.inner.borrow().get_property(name);

        Self::to_py(py, prop)
    }

    pub fn set_property(
        &mut self,
        name: String,
        value: &Bound<PyAny>,
        py: Python,
    ) -> PyResult<Py<PyAny>> {
        let val = Self::to_rust(value)?;

        self.inner.borrow_mut().set_property(name, val);

        Ok(py.None())
    }
}

impl PyObjectWrapper {
    pub fn new(inner: Rc<RefCell<Box<dyn Object>>>) -> Self {
        Self { inner }
    }

    pub fn to_py(py: Python, value: Box<dyn Any>) -> PyResult<Py<PyAny>> {
        for handled in [
            value.downcast_ref::<i32>().map(|v| v.into_py_any(py)),
            value.downcast_ref::<f32>().map(|v| v.into_py_any(py)),
            value.downcast_ref::<bool>().map(|v| v.into_py_any(py)),
            value.downcast_ref::<String>().map(|v| v.into_py_any(py)),
            value.downcast_ref::<&str>().map(|v| v.into_py_any(py)),
        ] {
            if let Some(result) = handled {
                return result;
            }
        }

        if let Some(v) = value.downcast_ref::<Vector2>() {
            return (v.x, v.y).into_py_any(py);
        }

        if let Some(v) = value.downcast_ref::<Color>() {
            return (v.r, v.g, v.b, v.a).into_py_any(py);
        }

        Ok(py.None())
    }

    pub fn to_rust(value: &Bound<PyAny>) -> PyResult<Box<dyn Any>> {
        for extractor in [
            value.extract::<i32>().map(|v| Box::new(v) as Box<dyn Any>),
            value.extract::<f32>().map(|v| Box::new(v) as Box<dyn Any>),
            value
                .extract::<String>()
                .map(|v| Box::new(v) as Box<dyn Any>),
            value.extract::<bool>().map(|v| Box::new(v) as Box<dyn Any>),
        ] {
            if let Ok(v) = extractor {
                return Ok(v);
            }
        }

        if let Ok(v) = value.extract::<(i32, i32)>() {
            return Ok(Box::new(Vector2::from(v)) as Box<dyn Any>);
        }

        if let Ok((r, g, b, a)) = value.extract::<(u8, u8, u8, u8)>() {
            return Ok(Box::new(Color::new(r, g, b, a)) as Box<dyn Any>);
        }

        if value.is_none() {
            return Ok(Box::new(()) as Box<dyn Any>);
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
            "Unsupported Python type: {}",
            value
        )))
    }
}

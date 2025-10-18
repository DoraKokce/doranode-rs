use raylib::prelude::*;

use crate::structs::Vector2;

pub trait Object {
    fn z_index(&self) -> i32;
    fn position(&self) -> Vector2;
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
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

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
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
#[derive(Debug)]
pub struct Text {
    pub position: Vector2,
    pub foreground_color: Color,
    pub font: Font,
    pub font_size: f32,
    pub text: String,
    pub z: i32,
}

impl Object for Text {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let rl_vec: raylib::prelude::Vector2 = raylib::prelude::Vector2::from(&self.position);

        draw_handle.draw_text_ex(
            &self.font,
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
    pub path: String,
    pub texture: Option<Texture2D>,
    pub z: i32,
}

impl Image {
    pub fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(tex) = rl.load_texture(thread, &self.path) {
            self.texture = Some(tex);
        } else {
            eprintln!("[-] Resim {} yüklenemedi", self.path);
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

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        if let Some(tex) = &self.texture {
            draw_handle.draw_texture(tex, 100, 100, Color::WHITE);
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

impl Into<ffi::Camera2D> for Camera {
    fn into(self) -> ffi::Camera2D {
        ffi::Camera2D {
            offset: self.offset.into(),
            target: self.target.into(),
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

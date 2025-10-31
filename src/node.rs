use std::{any::Any, cell::RefCell, rc::Rc};

use raylib::prelude::*;
use raylib_sys::{CheckCollisionPointRec, rlPopMatrix, rlPushMatrix, rlTranslatef};

use crate::{
    node_translations::Translations,
    objects::{self, Camera, Object},
    structs::Vector2,
};

pub trait TypeColor {
    const COLOR: Color;
}
impl TypeColor for i32 {
    const COLOR: Color = Color::ORANGE;
}

impl TypeColor for f32 {
    const COLOR: Color = Color::ORANGERED;
}

impl TypeColor for String {
    const COLOR: Color = Color::GREEN;
}

impl TypeColor for bool {
    const COLOR: Color = Color::YELLOW;
}

impl TypeColor for () {
    const COLOR: Color = Color::GRAY;
}

pub trait AnyPort {
    fn color(&self) -> Color;
    fn y_offset(&self) -> i32;
    fn as_any(&self) -> &dyn Any;
    fn set_position(&mut self, position: Vector2);
    fn get_position(&self) -> Vector2;
    fn read(&self) -> Option<&dyn Any>;
    fn write(&mut self, data: Option<&dyn Any>);
}

pub struct Port<T: Clone> {
    pub y_offset: i32,
    pub color: Color,
    pub data: Option<T>,
    pub position: Vector2,
}

impl<T: Clone + TypeColor> Port<T> {
    pub fn new(offset: i32, color: Option<Color>) -> Self {
        Self {
            y_offset: offset,
            color: color.unwrap_or(<T as TypeColor>::COLOR),
            data: None,
            position: Vector2::zero(),
        }
    }
}

impl<T: Clone + TypeColor + 'static> AnyPort for Port<T> {
    fn color(&self) -> Color {
        self.color
    }

    fn y_offset(&self) -> i32 {
        self.y_offset
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn get_position(&self) -> Vector2 {
        self.position.clone()
    }

    fn read(&self) -> Option<&dyn Any> {
        self.data.as_ref().map(|d| d as &dyn Any)
    }

    fn write(&mut self, data: Option<&dyn Any>) {
        if let Some(d) = data {
            if let Some(value) = d.downcast_ref::<T>() {
                self.data = Some(value.clone());
            }
        }
    }
}

pub struct Connection {
    pub from: Rc<RefCell<Box<dyn AnyPort>>>,
    pub to: Rc<RefCell<Box<dyn AnyPort>>>,
    pub color: Color,
    pub z: i32,
}

impl Object for Connection {
    fn z_index(&self) -> i32 {
        self.z
    }

    fn position(&self) -> Vector2 {
        Vector2::zero()
    }

    fn set_position(&mut self, _position: Vector2) {}

    fn update(&mut self, rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread, camera: &Camera) {
        let from = self.from.borrow();
        let mut to = self.to.borrow_mut();

        to.write(from.read());
        self.color = from.color();
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _camera: &Camera) {
        let from_pos = self.from.borrow().get_position().clone();
        let to_pos = self.to.borrow().get_position().clone();

        draw_handle.draw_line_bezier(from_pos, to_pos, 3.0, self.color);
    }
}

pub struct Node {
    pub positon: Vector2,
    pub size: Vector2,
    pub background_color: Color,
    pub border_color: Option<Color>,
    pub foreground_color: Color,
    pub components: Vec<Box<dyn Object>>,
    pub ports: Vec<(String, bool, Rc<RefCell<Box<dyn AnyPort>>>)>,
    pub active_color: Option<Color>,
    pub mouse_offset: Option<Vector2>,
    pub active: bool,
    pub roundness: f32,
    pub font: Rc<Font>,
    pub title_height: f32,
    pub update_fn: Option<Box<dyn Fn(&mut Node) + 'static>>,
    pub draw_fn: Option<Box<dyn Fn(&Node, &mut RaylibDrawHandle, Camera) + 'static>>,
    pub type_name: &'static str,
    pub translations: Rc<RefCell<Translations>>,
    pub language: Rc<RefCell<String>>,
    pub z: i32,
}

impl Object for Node {
    fn z_index(&self) -> i32 {
        self.z
    }
    fn position(&self) -> Vector2 {
        self.positon.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.positon = position;
    }

    fn update(&mut self, rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread, camera: &Camera) {
        let mouse_pos =
            Vector2::from(rl_handle.get_screen_to_world2D(rl_handle.get_mouse_position(), camera));
        let rect = raylib_sys::Rectangle {
            x: self.positon.x as f32,
            y: self.positon.y as f32 - self.title_height - 2.0,
            width: self.size.x as f32,
            height: self.size.y as f32 + 2.0,
        };

        let origin = self.positon.from_origin() + Vector2::new(5, 5, None);
        for component in &mut self.components {
            component.set_position(origin.clone());
            component.update(rl_handle, rl_thread, camera);
        }

        /* Drag and drop */
        if rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if unsafe { CheckCollisionPointRec(mouse_pos.clone().into(), rect) } {
                self.active = true;
                self.mouse_offset = Some(mouse_pos.clone() - self.positon.clone());
            }
        }

        if rl_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.active {
                if let Some(offset) = self.mouse_offset.clone() {
                    self.positon = mouse_pos.clone() - offset;
                }
            }
        } else {
            self.active = false;
            self.mouse_offset = None;
        }

        for (_, is_output, port) in self.ports.iter() {
            let mut port_borrow = port.borrow_mut();
            let port_position = Vector2::new(
                self.positon.x + if *is_output { self.size.x } else { 0 },
                self.positon.y + port_borrow.y_offset() + 6,
                None,
            );

            port_borrow.set_position(port_position.clone());
            if (port_position.clone() - mouse_pos.clone()).magnitude() <= 6.0 {
                self.active = false;
            }
        }

        if let Some(update_fn) = self.update_fn.take() {
            update_fn(self);
            self.update_fn = Some(update_fn);
        }
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera) {
        let rect = Rectangle::new(
            self.positon.x as f32,
            self.positon.y as f32 - self.title_height,
            self.size.x as f32,
            self.size.y as f32 + self.title_height,
        );

        draw_handle.draw_rectangle_rounded(rect, self.roundness, 32, self.background_color);
        //draw_handle.draw_rectangle_pro(rect, Vector2::zero(), 0.0, self.background_color);

        let border_color = self.border_color.unwrap_or(self.background_color);
        let active_border_color = self.active_color.unwrap_or(self.background_color);
        draw_handle.draw_rectangle_rounded_lines_ex(
            rect,
            self.roundness,
            32,
            3.0,
            if self.active {
                active_border_color
            } else {
                border_color
            },
        );
        //draw_handle.draw_rectangle_lines_ex(rect, 3.0, border_color);

        /* Draw Title */
        draw_handle.draw_line_ex(
            self.positon.clone(),
            self.positon.clone() + Vector2::new(self.size.x, 0, None),
            3.0,
            if self.active {
                active_border_color
            } else {
                border_color
            },
        );

        let title = self
            .translations
            .borrow()
            .get_node_translation(self.language.borrow().clone().as_str(), self.type_name)
            .title;
        let text_size = self.title_height - 2.0;
        let text_spacing = self.font.measure_text(&title, text_size, 1.0);
        let text_pos = Vector2::new(
            self.positon.x + (self.size.x - text_spacing.x as i32) / 2,
            self.positon.y - self.title_height as i32
                + (self.title_height as i32 - text_spacing.y as i32) / 2,
            None,
        );
        draw_handle.draw_text_ex(
            &*self.font,
            &title,
            text_pos,
            text_size,
            1.0,
            self.foreground_color,
        );

        /* Custom Draw Function */
        if let Some(draw_fn) = &self.draw_fn {
            unsafe {
                rlPushMatrix();
                rlTranslatef(self.positon.x as f32, self.positon.y as f32 + 2.0, 0.0);
            }

            let screen_pos = draw_handle.get_world_to_screen2D(self.positon.clone(), camera);
            draw_handle.draw_scissor_mode(
                screen_pos.x as i32,
                screen_pos.y as i32,
                self.size.x,
                self.size.y,
                |mut scissor: RaylibScissorMode<'_, RaylibDrawHandle<'_>>| {
                    draw_fn(self, &mut scissor, camera.clone());
                },
            );

            unsafe {
                rlPopMatrix();
            }
        }

        /* Draw Components */
        let screen_pos =
            draw_handle.get_world_to_screen2D(self.positon.clone(), Camera2D::from(camera.clone()));
        draw_handle.draw_scissor_mode(
            screen_pos.x as i32 + 5,
            screen_pos.y as i32 + 5,
            self.size.x - 10,
            self.size.y - 10,
            |mut scissor| {
                for component in &self.components {
                    component.draw(&mut scissor, camera);
                }
            },
        );

        /* Draw inputs and outputs */
        for (label, is_output, port) in self.ports.iter() {
            let port_borrow = port.borrow();
            let port_pos = port_borrow.get_position();

            draw_handle.draw_circle_v(
                Vector2::new(port_pos.x, port_pos.y, None),
                6.0,
                port_borrow.color(),
            );

            draw_handle.draw_circle_lines(port_pos.x, port_pos.y, 6.0, border_color);

            let text_size = 16.0;
            let text_spacing = self.font.measure_text(label, text_size, 1.0);
            let text_pos = Vector2::new(
                port_pos.x
                    + if *is_output {
                        -text_spacing.x as i32 - 10
                    } else {
                        10
                    },
                port_pos.y - (text_spacing.y as i32) / 2,
                None,
            );
            draw_handle.draw_text_ex(
                &*self.font,
                label,
                text_pos,
                text_size,
                1.0,
                self.foreground_color,
            );
        }
    }
}

impl Node {
    pub fn new(
        position: Vector2,
        size: Vector2,
        font: Rc<Font>,
        background_color: Color,
        border_color: Option<Color>,
        foreground_color: Color,
        active_color: Option<Color>,
        update_fn: Option<Box<dyn Fn(&mut Node) + 'static>>,
        draw_fn: Option<Box<dyn Fn(&Node, &mut RaylibDrawHandle, Camera) + 'static>>,
        type_name: &'static str,
        translations: Rc<RefCell<Translations>>,
        language: Rc<RefCell<String>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            positon: position,
            size,
            background_color,
            border_color,
            foreground_color,
            components: vec![],
            active_color: Some(active_color.unwrap_or(Color::ORANGE)),
            active: false,
            roundness: 0.2,
            font: font,
            title_height: 24.0,
            mouse_offset: None,
            update_fn,
            draw_fn,
            type_name,
            ports: vec![],
            language,
            translations,
            z: 0,
        }))
    }

    pub fn as_rc(&self) -> Rc<RefCell<Box<&Self>>> {
        Rc::new(RefCell::new(Box::new(self)))
    }

    pub fn add_port(
        this: &Rc<RefCell<Self>>,
        port: Box<dyn AnyPort>,
        label: &str,
        is_output: bool,
    ) {
        this.borrow_mut()
            .ports
            .push((label.to_string(), is_output, Rc::new(RefCell::new(port))));
    }

    pub fn add_ports(this: &Rc<RefCell<Self>>, ports: Vec<(Box<dyn AnyPort>, &str, bool)>) {
        for (port, label, is_output) in ports {
            Node::add_port(this, port, label, is_output);
        }
    }

    pub fn get_port(&self, label: &str, is_output: bool) -> Option<Rc<RefCell<Box<dyn AnyPort>>>> {
        for (port_label, port_is_output, port) in &self.ports {
            if port_label == label && *port_is_output == is_output {
                return Some(port.clone());
            }
        }
        None
    }

    pub fn read_typed_port<T: 'static + Clone>(
        node: &Node,
        label: &str,
        is_output: bool,
    ) -> Option<T> {
        node.get_port(label, is_output).and_then(|port_rc| {
            let port_ref = port_rc.borrow();
            port_ref
                .read()
                .and_then(|any_value| any_value.downcast_ref::<T>().cloned())
        })
    }

    pub fn write_typed_port<T: 'static + Clone>(
        node: &Node,
        label: &str,
        value: T,
        is_output: bool,
    ) {
        if let Some(port_rc) = node.get_port(label, is_output) {
            let mut port_ref = port_rc.borrow_mut();
            let any_ref = &value as &dyn Any;
            port_ref.write(Some(any_ref));
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Object>) {
        self.components.push(component);
    }
}

impl Into<Rectangle> for Node {
    fn into(self) -> Rectangle {
        Rectangle {
            x: (self.positon.x - self.size.x / 2) as f32,
            y: (self.positon.y - self.size.y / 2) as f32,
            width: self.size.x as f32,
            height: self.size.y as f32,
        }
    }
}

impl Into<objects::Rectangle> for Node {
    fn into(self) -> objects::Rectangle {
        objects::Rectangle {
            background_color: self.background_color,
            border_color: self.border_color,
            border_thickness: Some(3),
            position: self.positon,
            size: self.size,
            z: self.z,
        }
    }
}

impl Into<objects::RoundedRectangle> for Node {
    fn into(self) -> objects::RoundedRectangle {
        objects::RoundedRectangle {
            background_color: self.background_color,
            border_color: self.border_color,
            border_thickness: Some(3),
            position: self.positon,
            roundness: self.roundness,
            size: self.size,
            z: self.z,
        }
    }
}

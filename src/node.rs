use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use pyo3::{
    IntoPyObjectExt, PyAny, PyClass,
    ffi::c_str,
    prelude::*,
    types::{PyBool, PyDict, PyFloat, PyInt, PyNone, PyString},
};
use raylib::prelude::*;
use raylib_sys::{CheckCollisionPointRec, rlPopMatrix, rlPushMatrix, rlTranslatef};

use crate::{
    objects::{self, Camera, Object},
    structs::Vector2,
    translations::Translations,
};

pub struct Port {
    data: Py<PyAny>,
    position: Vector2,
    border_color: Color,
}

impl Object for Port {
    fn position(&self) -> Vector2 {
        self.position.clone()
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn z_index(&self) -> i32 {
        0
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera) {
        draw_handle.draw_circle(self.position.x, self.position.y, 6.0, Color::ORANGE);
        draw_handle.draw_circle_lines(self.position.x, self.position.y, 6.0, self.border_color);
    }
}

impl Port {
    pub fn new(border_color: Color) -> Self {
        Python::attach(|py| Self {
            data: py.None().into(),
            position: Vector2::zero(),
            border_color,
        })
    }

    fn read(&self, py: Python<'_>) -> Py<PyAny> {
        self.data.clone_ref(py)
    }

    fn write(&mut self, value: Py<PyAny>) {
        self.data = value;
    }
}

pub struct Connection {
    pub from: Rc<RefCell<Port>>,
    pub to: Rc<RefCell<Port>>,
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

    fn update(&mut self, _: &mut RaylibHandle, _: &RaylibThread, _: &Camera) {
        let from = self.from.borrow();
        let mut to = self.to.borrow_mut();

        Python::attach(|py| {
            to.data = from.data.clone_ref(py);
        });
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _camera: &Camera) {
        let from_pos = self.from.borrow().position();
        let to_pos = self.to.borrow().position();

        draw_handle.draw_line_bezier(from_pos, to_pos, 3.0, Color::ORANGE);
    }
}

pub struct Node {
    pub positon: Vector2,
    pub size: Vector2,
    pub background_color: Color,
    pub border_color: Option<Color>,
    pub foreground_color: Color,
    pub components: Vec<Box<dyn Object>>,
    pub ports: Vec<(String, bool, i32, Rc<RefCell<Box<Port>>>)>,
    pub active_color: Option<Color>,
    pub mouse_offset: Option<Vector2>,
    pub active: bool,
    pub roundness: f32,
    pub font: Rc<Font>,
    pub title_height: f32,
    pub update_fn: Option<Py<PyAny>>,
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

        for (_, is_output, y_offset, port) in self.ports.iter() {
            let mut port_borrow = port.borrow_mut();
            let port_position = Vector2::new(
                self.positon.x + if *is_output { self.size.x } else { 0 },
                self.positon.y + y_offset + 6,
                None,
            );

            port_borrow.set_position(port_position.clone());
            if (port_position.clone() - mouse_pos.clone()).magnitude() <= 6.0 {
                self.active = false;
            }
        }

        if let Some(update_fn) = &self.update_fn {
            Python::attach(|py| {
                let inputs = self.get_inputs_py_dict(py);
                let outputs: Py<PyDict> = PyDict::new(py).into();

                let globals = PyDict::new(py);
                globals.set_item("inputs", &inputs);
                globals.set_item("outputs", &outputs);
                globals.set_item("update", self.update_fn.as_ref());

                py.run(c_str!("update()"), Some(&globals), None);

                let node_outputs: Vec<(String, &Rc<RefCell<Box<Port>>>)> = self.get_outputs();
                let outputs: &HashMap<String, Py<PyAny>> =
                    &outputs.extract::<HashMap<String, Py<PyAny>>>(py).unwrap();
                for (key, value) in outputs {
                    if let Some((_, port_rc)) =
                        node_outputs.iter().find(|(port_key, _)| port_key == key)
                    {
                        let mut port = port_rc.borrow_mut();

                        port.write(value.clone_ref(py));
                    }
                }
            });
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
        for (label, is_output, _, port) in self.ports.iter() {
            let port_pos = port.borrow().position();
            port.borrow().draw(draw_handle, camera);

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
        update_fn: Option<Py<PyAny>>,
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
        port: Box<Port>,
        label: &str,
        is_output: bool,
        y_offset: i32,
    ) {
        this.borrow_mut().ports.push((
            label.to_string(),
            is_output,
            y_offset,
            Rc::new(RefCell::new(port)),
        ));
    }

    pub fn add_ports(this: &Rc<RefCell<Self>>, ports: Vec<(Box<Port>, &str, bool, i32)>) {
        for (port, label, is_output, y_offset) in ports {
            Node::add_port(this, port, label, is_output, y_offset);
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Object>) {
        self.components.push(component);
    }

    pub fn get_inputs(&self) -> Vec<(String, &Rc<RefCell<Box<Port>>>)> {
        self.ports
            .iter()
            .filter(|(_, is_output, _, _)| !*is_output)
            .map(|(label, _, _, port)| (label.clone(), port))
            .collect()
    }

    pub fn get_outputs(&self) -> Vec<(String, &Rc<RefCell<Box<Port>>>)> {
        self.ports
            .iter()
            .filter(|(_, is_output, _, _)| *is_output)
            .map(|(label, _, _, port)| (label.clone(), port))
            .collect()
    }

    pub fn get_inputs_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);

        let inputs = self.get_inputs();

        for (key, port) in inputs {
            dict.set_item(key, port.borrow().read(py)).unwrap();
        }

        dict.into()
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

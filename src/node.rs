use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    f32,
    rc::{Rc, Weak},
};

use pyo3::{
    PyAny,
    prelude::*,
    types::{PyDict, PyTuple},
};
use raylib::prelude::*;
use raylib_sys::{CheckCollisionPointRec, rlPopMatrix, rlPushMatrix, rlTranslatef};

use crate::{
    colorscheme::ColorSchemes,
    objects::{self, Camera, Object, PyObjectWrapper},
    settings::Settings,
    structs::Vector2,
    translations::Translations,
    window::EDITOR_STATE,
};

pub struct Port {
    data: Py<PyAny>,
    pub position: Vector2,
    border_color: Color,
    pub parent_id: String,
    pub label: String,
}

impl Object for Port {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _: &Camera) {
        draw_handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            6.0,
            Color::ORANGE,
        );
        draw_handle.draw_circle_lines(
            self.position.x as i32,
            self.position.y as i32,
            6.0,
            self.border_color,
        );
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any>) {
        match key.as_str() {
            "position" => {
                if let Ok(v) = value.downcast::<Vector2>() {
                    self.position = *v;
                }
            }
            "border_color" => {
                if let Ok(v) = value.downcast::<Color>() {
                    self.border_color = *v;
                }
            }
            "data" => {
                if let Ok(v) = value.downcast::<Py<PyAny>>() {
                    self.data = *v;
                }
            }
            _ => {
                eprintln!("Port: bilinmeyen özellik '{}'", key);
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "border_color" => Box::new(self.border_color),
            _ => {
                eprintln!("Port: bilinmeyen özellik '{}'", key);
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

impl Port {
    pub fn new(border_color: Color) -> Self {
        Python::attach(|py| Self {
            data: py.None().into(),
            position: Vector2::zero(),
            border_color,
            parent_id: "".to_string(),
            label: "".to_string(),
        })
    }

    pub fn read(&self, py: Python<'_>) -> Py<PyAny> {
        self.data.clone_ref(py)
    }

    pub fn write(&mut self, value: Py<PyAny>) {
        self.data = value;
    }
}

pub struct Connection {
    pub from: Rc<RefCell<Box<Port>>>,
    pub to: Rc<RefCell<Box<Port>>>,
    pub z: i32,
}

impl Object for Connection {
    fn update(&mut self, rl: &mut RaylibHandle, _: &RaylibThread, _: &Camera) {
        let from = self.from.borrow();
        let mut to = self.to.borrow_mut();

        Python::attach(|py| {
            to.write(from.read(py));
        });
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, _camera: &Camera) {
        let from_pos = self.from.borrow().position.clone();
        let to_pos = self.to.borrow().position.clone();

        draw_handle.draw_line_bezier(from_pos, to_pos, 3.0, Color::ORANGE);
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any>) {
        match key.as_str() {
            "from" => {
                if let Ok(v) = value.downcast::<Rc<RefCell<Box<Port>>>>() {
                    self.from = *v;
                }
            }
            "to" => {
                if let Ok(v) = value.downcast::<Rc<RefCell<Box<Port>>>>() {
                    self.to = *v;
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => eprintln!("set_property: bilinmeyen anahtar '{}'", key),
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "from" => Box::new(Rc::clone(&self.from)),
            "to" => Box::new(Rc::clone(&self.to)),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("get_property: bilinmeyen anahtar '{}'", key);
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

pub struct Node {
    pub position: Vector2,
    pub size: Vector2,
    pub components: HashMap<String, (Vector2, Rc<RefCell<Box<dyn Object>>>)>,
    pub ports: Vec<(String, bool, i32, Rc<RefCell<Box<Port>>>)>,
    mouse_offset: Option<Vector2>,
    pub active: bool,
    pub roundness: f32,
    pub font: Rc<RefCell<Font>>,
    pub title_height: f32,
    pub update_fn: Option<Py<PyAny>>,
    pub draw_fn: Option<Box<dyn Fn(&Node, &mut RaylibDrawHandle, Camera) + 'static>>,
    pub type_name: &'static str,
    pub id: String,
    pub translations: Rc<RefCell<Translations>>,
    pub color_schemes: Rc<RefCell<ColorSchemes>>,
    pub settings: Rc<RefCell<Settings>>,
    pub rc_self: Option<Weak<RefCell<Node>>>,
    pub scalable: bool,
    pub z: i32,
}

impl Object for Node {
    fn update(&mut self, rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread, camera: &Camera) {
        let mouse_pos =
            Vector2::from(rl_handle.get_screen_to_world2D(rl_handle.get_mouse_position(), camera));
        let rect = raylib_sys::Rectangle {
            x: self.position.x,
            y: self.position.y - self.title_height - 2.0,
            width: self.size.x,
            height: self.size.y + 2.0,
        };

        let origin = self.position.from_origin() + Vector2::new(5.0, 5.0, None);
        for (_, (offset, component)) in &mut self.components {
            component.borrow_mut().set_property(
                "position".to_string(),
                Box::new(origin.clone() + offset.clone()),
            );
            component.borrow_mut().update(rl_handle, rl_thread, camera);
        }

        /* Drag and drop */
        if rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            || rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            if unsafe { CheckCollisionPointRec(mouse_pos.clone().into(), rect) } {
                self.active = true;
                self.mouse_offset = Some(mouse_pos.clone() - self.position.clone());
            }
        }

        if rl_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            || rl_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            if self.active {
                if let Some(offset) = self.mouse_offset.clone() {
                    self.position = mouse_pos.clone() - offset;
                }
            }
        } else {
            self.active = false;
            self.mouse_offset = None;
        }

        for (i, (label, is_output, y_offset, port)) in self.ports.iter().enumerate() {
            let port_position = Vector2::new(
                self.position.x + if *is_output { self.size.x } else { 0.0 },
                self.position.y + *y_offset as f32,
                None,
            );

            {
                let mut port_borrow = port.borrow_mut();
                port_borrow.set_property("position".to_string(), Box::new(port_position.clone()));
                port_borrow.parent_id = self.id.to_string();
                port_borrow.label = label.clone();
            }

            if (port_position.clone() - mouse_pos.clone()).magnitude() <= 6.0 {
                self.active = false;

                if rl_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    EDITOR_STATE.with(|state| {
                        let mut state = state.borrow_mut();
                        if *is_output && state.dragging_from.is_none() {
                            state.dragging_from = Some(port.clone());
                        } else if !*is_output {
                            state.dragging_to = Some(port.clone());
                        }
                    });
                }
            }
        }

        if let Some(update_fn) = self.update_fn.take() {
            let _ = Python::attach(|py| -> PyResult<()> {
                let inputs = self.get_inputs_py_dict(py);

                let kwargs = PyDict::new(py);
                kwargs.set_item("inputs", &inputs)?;
                kwargs.set_item("components", self.get_components_py_dict(py))?;

                let outputs: HashMap<String, Py<PyAny>> = update_fn
                    .call(py, PyTuple::empty(py), Some(&kwargs))
                    .expect(
                        format!("Update function of {} throwed an error", self.type_name).as_str(),
                    )
                    .extract(py)
                    .unwrap_or(HashMap::new());

                for (label, value) in &outputs {
                    self.write_port(label, value.clone_ref(py));
                }

                Ok(())
            });
            self.update_fn = Some(update_fn);
        }

        self.fit_around_components();
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle, camera: &Camera) {
        let rect = Rectangle::new(
            self.position.x,
            self.position.y - self.title_height,
            self.size.x,
            self.size.y + self.title_height,
        );

        let schemes = self.color_schemes.borrow();
        let scheme = self.settings.borrow().scheme.to_string();
        let background_color = schemes
            .get_color(&scheme, "node_background")
            .unwrap_or(Color::MAGENTA);
        let border_color = schemes
            .get_color(&scheme, "node_border")
            .unwrap_or(Color::MAGENTA);
        let active_border_color = schemes
            .get_color(&scheme, "node_active_border")
            .unwrap_or(Color::MAGENTA);
        let foreground_color = schemes
            .get_color(&scheme, "node_foreground")
            .unwrap_or(Color::MAGENTA);

        draw_handle.draw_rectangle_rounded(rect, self.roundness, 32, background_color);
        //draw_handle.draw_rectangle_pro(rect, Vector2::zero(), 0.0, self.background_color);

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
            self.position.clone(),
            self.position.clone() + Vector2::new(self.size.x, 0.0, None),
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
            .get_node_translation(self.settings.borrow().language.as_str(), self.type_name)
            .title;
        let text_size = self.title_height - 2.0;
        let text_spacing = self.font.borrow().measure_text(&title, text_size, 1.0);
        let text_pos = Vector2::new(
            self.position.x + (self.size.x - text_spacing.x) / 2.0,
            self.position.y - self.title_height + (self.title_height - text_spacing.y) / 2.0,
            None,
        );
        draw_handle.draw_text_ex(
            &*self.font.borrow(),
            &title,
            text_pos,
            text_size,
            1.0,
            foreground_color,
        );

        /* Custom Draw Function */
        if let Some(draw_fn) = &self.draw_fn {
            unsafe {
                rlPushMatrix();
                rlTranslatef(self.position.x, self.position.y + 2.0, 0.0);
            }

            let screen_pos = draw_handle.get_world_to_screen2D(self.position.clone(), camera);
            draw_handle.draw_scissor_mode(
                screen_pos.x as i32,
                screen_pos.y as i32,
                (self.size.x * camera.zoom) as i32,
                (self.size.y * camera.zoom) as i32,
                |mut scissor: RaylibScissorMode<'_, RaylibDrawHandle<'_>>| {
                    draw_fn(self, &mut scissor, camera.clone());
                },
            );

            unsafe {
                rlPopMatrix();
            }
        }

        /* Draw Components */
        let screen_pos = draw_handle
            .get_world_to_screen2D(self.position.clone(), Camera2D::from(camera.clone()));
        draw_handle.draw_scissor_mode(
            screen_pos.x as i32 + 5,
            screen_pos.y as i32 + 5,
            ((self.size.x - 10.0) * camera.zoom) as i32,
            ((self.size.y - 10.0) * camera.zoom) as i32,
            |mut scissor| {
                for (_, (_, component)) in &self.components {
                    let mut comp = component.borrow_mut();
                    comp.set_property(
                        "background_color".to_string(),
                        Box::new(
                            schemes
                                .get_color(&scheme, "node_component_background")
                                .unwrap(),
                        ),
                    );
                    comp.set_property(
                        "foreground_color".to_string(),
                        Box::new(
                            schemes
                                .get_color(&scheme, "node_component_foreground")
                                .unwrap(),
                        ),
                    );
                    comp.set_property(
                        "active_background_color".to_string(),
                        Box::new(
                            schemes
                                .get_color(&scheme, "node_component_active_background")
                                .unwrap(),
                        ),
                    );
                    comp.draw(&mut scissor, camera);
                }
            },
        );

        /* Draw inputs and outputs */
        for (label, is_output, _, port) in self.ports.iter() {
            let port_pos = port.borrow().position.clone();
            port.borrow().draw(draw_handle, camera);

            let text_size = 16.0;
            let text_spacing = self.font.borrow().measure_text(label, text_size, 1.0);
            let text_pos = Vector2::new(
                port_pos.x
                    + if *is_output {
                        -text_spacing.x - 10.0
                    } else {
                        10.0
                    },
                port_pos.y - text_spacing.y / 2.0,
                None,
            );
            draw_handle.draw_text_ex(
                &*self.font.borrow(),
                label,
                text_pos,
                text_size,
                1.0,
                foreground_color,
            );
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
            "active" => {
                if let Ok(v) = value.downcast::<bool>() {
                    self.active = *v;
                }
            }
            "roundness" => {
                if let Ok(v) = value.downcast::<f32>() {
                    self.roundness = *v;
                }
            }
            "z" => {
                if let Ok(v) = value.downcast::<i32>() {
                    self.z = *v;
                }
            }
            _ => eprintln!("set_property: bilinmeyen anahtar '{}'", key),
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        match key.as_str() {
            "position" => Box::new(self.position.clone()),
            "size" => Box::new(self.size.clone()),
            "active" => Box::new(self.active),
            "roundness" => Box::new(self.roundness),
            "id" => Box::new(self.id.clone()),
            "type_name" => Box::new(self.type_name.to_string()),
            "ports" => Box::new(self.ports.clone()),
            "z" => Box::new(self.z),
            _ => {
                eprintln!("get_property: bilinmeyen anahtar '{}'", key);
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

impl Node {
    pub fn new(
        position: Vector2,
        size: Vector2,
        font: Rc<RefCell<Font>>,
        update_fn: Option<Py<PyAny>>,
        draw_fn: Option<Box<dyn Fn(&Node, &mut RaylibDrawHandle, Camera) + 'static>>,
        type_name: &'static str,
        translations: Rc<RefCell<Translations>>,
        color_schemes: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        id: String,
        scalable: bool,
    ) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Node {
            position,
            size,
            components: HashMap::new(),
            active: false,
            roundness: 0.2,
            font,
            title_height: 24.0,
            mouse_offset: None,
            update_fn,
            draw_fn,
            type_name,
            id,
            ports: vec![],
            translations,
            color_schemes,
            settings,
            scalable,
            rc_self: None,
            z: 0,
        }));

        {
            node.borrow_mut().rc_self = Some(Rc::downgrade(&node));
        }

        node
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

    pub fn add_component(
        this: &Rc<RefCell<Self>>,
        label: String,
        component: Box<dyn Object>,
        offset: Option<Vector2>,
    ) {
        this.borrow_mut().components.insert(
            label,
            (
                offset.unwrap_or(Vector2::zero()),
                Rc::new(RefCell::new(component)),
            ),
        );
    }

    pub fn get_inputs(&self) -> Vec<(String, &Rc<RefCell<Box<Port>>>)> {
        self.ports
            .iter()
            .filter(|(_, is_output, _, _)| !*is_output)
            .map(|(label, _, _, port)| (label.clone(), port))
            .collect()
    }

    pub fn get_inputs_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);

        let inputs = self.get_inputs();

        for (label, port) in inputs {
            dict.set_item(label, port.borrow().read(py)).unwrap();
        }

        dict.into()
    }

    pub fn get_components_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);

        for (label, (_offset, component)) in &self.components {
            dict.set_item(label, PyObjectWrapper::new(component.clone()))
                .unwrap();
        }

        dict.into()
    }

    pub fn write_port(&mut self, label: &str, value: Py<PyAny>) {
        self.ports
            .iter()
            .find(|(l, _, _, _)| l == label)
            .map(|(_, _, _, port)| port.borrow_mut().write(value));
    }

    pub fn read_port(&self, label: &str, py: Python) -> Option<Py<PyAny>> {
        self.ports
            .iter()
            .find(|(l, _, _, _)| l == label)
            .map(|(_, _, _, port)| port.borrow().read(py))
    }

    pub fn fit_around_components(&mut self) {
        if !self.scalable {
            return;
        }
        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;
        for (offset, component) in self.components.values() {
            let comp_size = component
                .borrow()
                .get_property("size".to_string())
                .downcast::<Vector2>()
                .unwrap();

            max_x = max_x.max(offset.x + comp_size.x + 5.0);
            max_y = max_y.max(offset.y + comp_size.y + 5.0);
        }

        self.size.x = max_x;
        self.size.y = max_y;
    }
}

impl Into<Rectangle> for Node {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size.x,
            height: self.size.y,
        }
    }
}

impl Into<objects::Rectangle> for Node {
    fn into(self) -> objects::Rectangle {
        objects::Rectangle {
            background_color: Color::WHITE,
            border_color: None,
            border_thickness: Some(3.0),
            position: self.position,
            size: self.size,
            z: self.z,
        }
    }
}

impl Into<objects::RoundedRectangle> for Node {
    fn into(self) -> objects::RoundedRectangle {
        objects::RoundedRectangle {
            background_color: Color::WHITE,
            border_color: None,
            border_thickness: Some(3),
            position: self.position,
            roundness: self.roundness,
            size: self.size,
            z: self.z,
        }
    }
}

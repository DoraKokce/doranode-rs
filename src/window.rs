use pyo3::{ffi::c_str, prelude::*};
use raylib::prelude::*;
use raylib_sys::{LoadImage, SetTextureFilter};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fs,
    rc::Rc,
};

use crate::{
    colorscheme::ColorSchemes,
    node::{Connection, Node, Port},
    node_gui,
    node_libary::NodeLibary,
    objects::{Camera, ComboBox, Grid, Object},
    settings::Settings,
    structs::Vector2,
    translations::Translations,
};

pub struct EditorState {
    pub dragging_from: Option<Rc<RefCell<Box<Port>>>>,
    pub dragging_to: Option<Rc<RefCell<Box<Port>>>>,
    pub connections: HashMap<String, Connection>,
    pub node_names: HashMap<String, Vec<usize>>,
    pub selected_module: Option<String>,
}

thread_local! {
    pub static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState {
        dragging_from: None,
        dragging_to: None,
        connections: HashMap::new(),
        node_names: HashMap::new(),
        selected_module: None,
    });
}

pub struct Window {
    pub fonts: HashMap<String, Rc<RefCell<Font>>>,
    pub active_font: Option<Rc<RefCell<Font>>>,
    pub translations: Rc<RefCell<Translations>>,
    pub color_schemes: Rc<RefCell<ColorSchemes>>,
    pub camera: Rc<RefCell<Camera>>,
    pub objects: HashMap<String, Rc<RefCell<dyn Object>>>,
    pub settings: Rc<RefCell<Settings>>,
    pub dragging: bool,
    pub last_mouse: Vector2,
    pub node_active: bool,
    pub lib: Rc<RefCell<NodeLibary>>,
    pub node_selector: Option<node_gui::NodeSelector>,
}

const TURKISH_ALPHABET: &str = " ABCDEFGHIİJKLMNOÖPRSŞTUÜVYZQWXYZabcdefghijklmnopqrstuvwxyzçğıöşüÇĞİÖŞÜ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~√";

impl Window {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            active_font: None,
            translations: Translations::new(),
            color_schemes: ColorSchemes::new(),
            camera: Rc::new(RefCell::new(Camera {
                offset: Vector2::new(320f32, 280f32, None),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            })),
            objects: HashMap::new(),
            settings: Rc::new(RefCell::new(Settings::load_settings("settings.toml"))),
            dragging: false,
            last_mouse: Vector2::zero(),
            node_active: false,
            lib: Rc::new(RefCell::new(NodeLibary::insert_default_nodes())),
            node_selector: None,
        }
    }

    pub fn init(&mut self) -> (RaylibHandle, RaylibThread) {
        Python::initialize();
        self.load_translations();
        self.load_schemes();

        let (mut rl_handle, rl_thread) = raylib::init().size(640, 480).build();
        rl_handle.set_window_title(
            &rl_thread,
            &self
                .translations
                .borrow()
                .get_gui_translation(&self.settings.borrow().language, "window.title"),
        );

        let logo = raylib::prelude::Image::load_image("resources/images/logo.png")
            .expect("logo yüklenemedi");
        rl_handle.set_window_icon(logo);
        rl_handle.set_exit_key(None);

        self.load_fonts(&mut rl_handle, &rl_thread);

        self.active_font = Some(self.fonts.get("Roboto-Regular").unwrap().clone());

        let settings = self.settings.borrow();

        self.objects.insert(
            "grid".to_string(),
            Rc::new(RefCell::new(Grid {
                background_color: Color::new(40, 40, 40, 255),
                big_square_color: Some(Color::new(80, 80, 80, 255)),
                big_square_size: Some(Vector2::new(4.0, 4.0, None)),
                position: -Vector2::from((
                    settings.grid_size[0] * settings.grid_square_size[0] / 2.0,
                    settings.grid_size[1] * settings.grid_square_size[1] / 2.0,
                )),
                size: settings.grid_size.clone().into(),
                square_color: Color::new(64, 64, 64, 255),
                square_size: settings.grid_square_size.clone().into(),
                z: -100,
            })),
        );

        self.node_selector = Some(node_gui::NodeSelector::new(
            self.lib.clone(),
            self.active_font.clone().unwrap(),
            self.color_schemes.clone(),
            self.settings.clone(),
            self.translations.clone(),
        ));

        (rl_handle, rl_thread)
    }

    pub fn run(&mut self, mut rl_handle: RaylibHandle, rl_thread: RaylibThread) {
        while !rl_handle.window_should_close() {
            self.update(&mut rl_handle, &rl_thread);
            self.draw(&mut rl_handle, &rl_thread);
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mouse: Vector2 = rl.get_mouse_position().into();
        let mut cam = self.camera.borrow_mut();
        let settings = self.settings.borrow();

        if rl.is_window_resized() {
            cam.offset = Vector2::new(
                rl.get_screen_width() as f32 / 2.0,
                rl.get_screen_height() as f32 / 2.0,
                None,
            );
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.dragging = true;
            self.last_mouse = mouse.clone();
        }

        EDITOR_STATE.with(|state| {
            let mut state = state.borrow_mut();

            if self.dragging
                && !self.node_active
                && state.dragging_from.is_none()
                && state.dragging_to.is_none()
                && state.selected_module.is_none()
            {
                let delta = mouse.clone() - self.last_mouse.clone();
                cam.target = cam.target.clone() - (delta / cam.zoom).into();
                self.last_mouse = mouse.clone();
            }

            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.dragging = false;
                if let Some(module) = state.selected_module.clone() {
                    let mut list = state
                        .node_names
                        .entry(module.clone())
                        .or_insert_with(Vec::new);

                    let next_index = if list.is_empty() {
                        1
                    } else {
                        Window::missing_numbers(list)
                            .first()
                            .cloned()
                            .unwrap_or_else(|| list.iter().max().cloned().unwrap() + 1)
                    };

                    let id = format!("{}{}", module, next_index);

                    if let Some(node) = self.lib.borrow().generate(
                        &module,
                        self.active_font.clone().unwrap(),
                        self.translations.clone(),
                        self.color_schemes.clone(),
                        self.settings.clone(),
                        id.clone(),
                    ) {
                        let mouse_world: Vector2 =
                            rl.get_screen_to_world2D(mouse.clone(), &cam.clone()).into();
                        node.borrow_mut().positon = mouse_world;

                        self.objects.insert(id.clone(), node.clone());

                        list.push(next_index);
                    }

                    state.selected_module = None;
                }
            }

            if let (Some(from), Some(to)) = (state.dragging_from.clone(), state.dragging_to.clone())
            {
                let from_name = from.borrow().parent_id.clone();
                let from_id = from.borrow().id;
                let to_name = to.borrow().parent_id.clone();
                let to_id = to.borrow().id;
                let namefrom = format!("({}({}))", from_name, from_id);
                let nameto = format!("({}({}))", to_name, to_id);
                let name = namefrom.clone() + &nameto;

                let mut to_remove: Vec<String> = vec![];

                for connection_name in state.connections.keys() {
                    if *connection_name == name || from_name == to_name {
                        state.dragging_from = None;
                        state.dragging_to = None;
                        return;
                    }
                    if connection_name.ends_with(&nameto) {
                        to_remove.push(connection_name.clone())
                    }
                }

                for remove in to_remove {
                    state.connections.remove(&remove);
                    println!("deleted: {}", remove)
                }

                state
                    .connections
                    .insert(name, Connection { from, to, z: 1 });

                state.dragging_from = None;
                state.dragging_to = None;
            }

            for (_, connection) in &mut state.connections {
                connection.update(rl, thread, &cam);
            }
        });

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            let mouse_world_before: Vector2 =
                rl.get_screen_to_world2D(mouse.clone(), &cam.clone()).into();

            let zoom_factor = 1.0 + wheel * 0.1;
            cam.zoom *= zoom_factor;
            cam.zoom = cam.zoom.clamp(0.5, 5.0);

            let mouse_world_after: Vector2 =
                rl.get_screen_to_world2D(mouse.clone(), &cam.clone()).into();
            cam.target = cam.target.clone() + mouse_world_before - mouse_world_after;
        }

        let grid_size: Vector2 = settings.grid_size.into();
        let grid_square_size: Vector2 = settings.grid_square_size.into();
        let screen_size: Vector2 = (rl.get_screen_width(), rl.get_screen_height()).into();
        let world_min = Vector2::new(
            (-grid_size.x / 2.0) * grid_square_size.x,
            (-grid_size.y / 2.0) * grid_square_size.y,
            None,
        );
        let world_max = Vector2::new(
            (grid_size.x / 2.0) * grid_square_size.x,
            (grid_size.y / 2.0) * grid_square_size.y,
            None,
        );
        let half_screen = Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0, None) / cam.zoom;
        cam.target.x = cam
            .target
            .clone()
            .x
            .clamp(world_min.x + half_screen.x, world_max.x - half_screen.x);
        cam.target.y = cam
            .target
            .clone()
            .y
            .clamp(world_min.y + half_screen.y, world_max.y - half_screen.y);

        let mut active_index: Option<usize> = None;
        let mut to_remove: Vec<String> = vec![];

        if let Some(selector) = &mut self.node_selector {
            selector.update(rl, thread, &cam);
        }

        for (i, (key, obj)) in self.objects.iter().enumerate() {
            let mut obj_mut = obj.borrow_mut();

            obj_mut.update(rl, thread, &cam);

            if let Some(node) = obj_mut.as_any_mut().downcast_mut::<Node>() {
                if let Ok(active) = node.get_property("active".to_string()).downcast::<bool>() {
                    if !*active {
                        continue;
                    }
                }

                if let Some(previous) = active_index {
                    self.objects.values().collect::<Vec<_>>()[previous]
                        .borrow_mut()
                        .set_property("active".to_string(), Box::new(false));
                }

                active_index = Some(i);
                if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
                    to_remove.push(key.clone());
                    EDITOR_STATE.with(|state| {
                        let mut state = state.borrow_mut();
                        let mut to_remove: Vec<String> = vec![];
                        for (connection, _) in &mut state.connections {
                            if connection.contains(format!("({}(", node.id).as_str()) {
                                to_remove.push(connection.clone());
                            }
                        }

                        for remove in to_remove {
                            state.connections.remove(&remove);
                        }
                    })
                }
            } else if let Some(connection) = obj_mut.as_any_mut().downcast_mut::<Connection>() {
                let from_pos = connection.from.borrow().position.clone();
                let to_pos = connection.to.borrow().position.clone();

                if Self::point_in_rect_from_two_points(from_pos, to_pos, mouse.clone())
                    && rl.is_key_pressed(KeyboardKey::KEY_DELETE)
                {
                    to_remove.push(key.clone());
                }
            }

            self.node_active = active_index.is_some();
        }

        for key in to_remove {
            self.objects.remove(&key);
        }

        let colorscheme = self.color_schemes.borrow();
        let scheme = self.settings.borrow().scheme.clone();

        {
            let grid_obj = self.objects.get("grid").unwrap().clone();
            let mut grid = grid_obj.borrow_mut();

            grid.set_property(
                "background_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_background")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
            grid.set_property(
                "square_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_square")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
            grid.set_property(
                "big_square_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_big_square")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
        }
    }

    fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mouse = rl.get_screen_to_world2D(
            rl.get_mouse_position(),
            Camera2D::from(self.camera.borrow().clone()),
        );
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::WHITE);

        {
            let mut cam = d.begin_mode2D(self.camera.borrow().clone());

            let mut sorted_objs: Vec<Rc<RefCell<dyn Object>>> =
                self.objects.values().cloned().collect();

            sorted_objs.sort_by(|a, b| {
                let az = a
                    .borrow()
                    .get_property("z".into())
                    .downcast::<i32>()
                    .map(|val| *val)
                    .unwrap_or(0);
                let bz = b
                    .borrow()
                    .get_property("z".into())
                    .downcast::<i32>()
                    .map(|val| *val)
                    .unwrap_or(0);
                az.cmp(&bz)
            });

            for obj in sorted_objs.into_iter() {
                obj.borrow().draw(&mut cam, &self.camera.borrow());
            }

            EDITOR_STATE.with(|state| {
                let state = state.borrow();
                for (_, connection) in &state.connections {
                    connection.draw(&mut cam, &self.camera.borrow());
                }
                if let Some(drag_port) = &state.dragging_from {
                    cam.draw_line_bezier(
                        drag_port.borrow().position.clone(),
                        mouse.clone(),
                        3.0,
                        self.color_schemes
                            .borrow()
                            .get_color(&self.settings.borrow().scheme, "connection_pending")
                            .unwrap(),
                    );
                }
            });
        }

        if let Some(selector) = &self.node_selector {
            selector.draw(&mut d, &self.camera.borrow());
        }
    }

    fn load_translations(&mut self) {
        if let Ok(translations) = fs::read_dir("resources/translations") {
            for translation in translations.flatten() {
                self.translations.borrow_mut().load_from_file(
                    fs::read_to_string(translation.path()).expect("translation cannot be loaded"),
                    &translation
                        .file_name()
                        .to_str()
                        .unwrap()
                        .replace(".json", ""),
                );
            }
        }
    }

    fn load_schemes(&mut self) {
        if let Ok(schemes) = fs::read_dir("resources/colorschemes") {
            for scheme in schemes.flatten() {
                self.color_schemes.borrow_mut().load(
                    fs::read_to_string(scheme.path()).expect("schemes cannot be loaded"),
                    scheme.file_name().to_str().unwrap().replace(".json", ""),
                );
            }
        }
    }

    fn load_fonts(&mut self, rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
        if let Ok(fonts) = fs::read_dir("resources/fonts") {
            for font in fonts.flatten() {
                let path = font.path();
                let path_str = path.to_str().unwrap();
                let loaded_font = rl
                    .load_font_ex(&rl_thread, path_str, 128, Some(TURKISH_ALPHABET))
                    .expect(
                        format!(
                            "Font yüklenemedi, Font adı: {}",
                            font.file_name().to_str().unwrap().to_string(),
                        )
                        .as_str(),
                    );
                unsafe {
                    SetTextureFilter(
                        loaded_font.texture,
                        raylib_sys::TextureFilter::TEXTURE_FILTER_TRILINEAR as i32,
                    )
                };
                self.fonts.insert(
                    font.file_name().to_str().unwrap().replace(".ttf", ""),
                    Rc::new(RefCell::new(loaded_font)),
                );
            }
        }
    }

    fn point_in_rect_from_two_points(p0: Vector2, p1: Vector2, p: Vector2) -> bool {
        let min_x = p0.x.min(p1.x);
        let max_x = p0.x.max(p1.x);
        let min_y = p0.y.min(p1.y);
        let max_y = p0.y.max(p1.y);

        p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
    }

    fn missing_numbers(vec: &Vec<usize>) -> Vec<usize> {
        let mut missing: Vec<usize> = Vec::new();
        if vec.is_empty() {
            return missing;
        }
        let min = *vec.iter().min().unwrap();
        let max = *vec.iter().max().unwrap();

        for i in min..=max {
            if !vec.contains(&i) {
                missing.push(i);
            }
        }
        missing
    }
}

use pyo3::prelude::*;
use raylib::prelude::*;
use raylib_sys::SetTextureFilter;
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

use crate::{
    colorscheme::ColorSchemes,
    gui::{self, Dialog, DialogButton, ToolBarItem},
    node::{Connection, Node, Port},
    node_libary::NodeLibary,
    objects::{Camera, Grid, Object},
    save::{CameraSave, NodeSave, SaveFile},
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
    pub dialog: Option<Dialog>,
    pub save_file: Option<SaveFile>,
    pub project_name: String,
}

thread_local! {
    pub static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState {
        dragging_from: None,
        dragging_to: None,
        connections: HashMap::new(),
        node_names: HashMap::new(),
        selected_module: None,
        dialog: None,
        save_file: None,
        project_name: "untitled".to_string(),
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
    pub node_selector: Option<gui::NodeSelector>,
    pub tool_bar: Option<gui::ToolBar>,
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
            tool_bar: None,
        }
    }

    pub fn init(&mut self) -> (RaylibHandle, RaylibThread) {
        Python::initialize();
        self.load_translations();
        self.load_schemes();

        let (mut rl_handle, rl_thread) = raylib::init().size(960, 720).resizable().build();
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
        rl_handle.set_target_fps(60);

        self.load_fonts(&mut rl_handle, &rl_thread);

        self.active_font = Some(self.fonts.get("Roboto-Regular").unwrap().clone());

        self.objects.insert("grid".to_string(), self.get_grid());

        self.node_selector = Some(gui::NodeSelector::new(
            self.lib.clone(),
            self.active_font.clone().unwrap(),
            self.color_schemes.clone(),
            self.settings.clone(),
            self.translations.clone(),
        ));

        let mut tool_bar = gui::ToolBar::new(
            self.color_schemes.clone(),
            self.settings.clone(),
            self.active_font.clone().unwrap(),
            self.translations.clone(),
        );

        tool_bar.add_item(ToolBarItem {
            label: "file".to_string(),
            children: vec![
                ToolBarItem {
                    label: "new".to_string(),
                    on_click: Some("new_file".to_string()),
                    children: vec![],
                    expanded: false,
                },
                ToolBarItem {
                    label: "open".to_string(),
                    on_click: Some("open_file".to_string()),
                    children: vec![],
                    expanded: false,
                },
                ToolBarItem {
                    label: "save".to_string(),
                    on_click: Some("save_file".to_string()),
                    children: vec![],
                    expanded: false,
                },
            ],
            on_click: None,
            expanded: false,
        });

        self.tool_bar = Some(tool_bar);

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

        EDITOR_STATE.with(|editor_state| {
            let mut state = editor_state.borrow_mut();

            if self.dragging
                && !self.node_active
                && state.dragging_from.is_none()
                && state.dragging_to.is_none()
                && state.selected_module.is_none()
                && state.dialog.is_none()
            {
                let delta = mouse.clone() - self.last_mouse.clone();
                cam.target = cam.target.clone() - (delta / cam.zoom).into();
                self.last_mouse = mouse.clone();
            }

            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.dragging = false;
                if let Some(module) = state.selected_module.clone() {
                    let list = state
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
                        node.borrow_mut().position = mouse_world;

                        self.objects.insert(id.clone(), node.clone());

                        list.push(next_index);
                    }

                    state.selected_module = None;
                }

                if let (Some(from), Some(to)) =
                    (state.dragging_from.clone(), state.dragging_to.clone())
                {
                    let from_name = from.borrow().parent_id.clone();
                    let from_id = from.borrow().label.clone();
                    let to_name = to.borrow().parent_id.clone();
                    let to_id = to.borrow().label.clone();
                    let namefrom = format!("(({}):(\"{}\"))", from_name, from_id);
                    let nameto = format!("(({}):(\"{}\"))", to_name, to_id);
                    let name = namefrom.clone() + &nameto;

                    let mut to_remove: Vec<String> = vec![];

                    for (connection_name, _) in state.connections.iter() {
                        if *connection_name == name || from_name == to_name {
                            state.dragging_from = None;
                            state.dragging_to = None;
                            return;
                        }
                        if connection_name.ends_with(&nameto) {
                            to_remove.push(connection_name.clone());
                        }
                    }

                    for remove_name in to_remove {
                        Self::remove_connection(&remove_name, &mut state);
                    }

                    state
                        .connections
                        .insert(name, Connection { from, to, z: 1 });
                }
                state.dragging_from = None;
                state.dragging_to = None;
            }

            let mut to_remove: Vec<String> = vec![];

            for (conn_name, conn) in &mut state.connections {
                conn.update(rl, thread, &cam);
                let from_pos = conn.from.borrow().position.clone();
                let to_pos = conn.to.borrow().position.clone();

                if Window::point_in_bezier_line(
                    from_pos.clone(),
                    to_pos.clone(),
                    rl.get_screen_to_world2D(mouse.clone(), &*cam).into(),
                    15.0,
                ) {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                        to_remove.push(conn_name.clone());
                    }
                }
            }

            for remove_name in to_remove {
                Self::remove_connection(&remove_name, &mut state);
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
            if let Some(tool_bar) = &mut self.tool_bar {
                tool_bar.position.x = selector.size.x;
                tool_bar.update(rl, thread, &cam);

                let events = std::mem::take(&mut tool_bar.events);

                for ev in events {
                    match ev.as_str() {
                        "save_file" => {
                            self.save_file(cam.clone());
                        }
                        "open_file" => self.open_file(),
                        "new_file" => self.new_file(),
                        _ => {}
                    }
                }
            }
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

                if rl.is_key_pressed(KeyboardKey::KEY_DELETE)
                    || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
                {
                    to_remove.push(key.clone());
                    EDITOR_STATE.with(|editor_state| {
                        let mut state = editor_state.borrow_mut();
                        let mut to_remove: Vec<String> = vec![];

                        for (connection, _) in &mut state.connections {
                            if connection.contains(node.id.as_str()) {
                                to_remove.push(connection.clone());
                            }
                        }

                        for remove_name in to_remove {
                            Self::remove_connection(&remove_name, &mut state);
                        }
                    })
                }

                active_index = Some(i);
            }

            self.node_active = active_index.is_some();
        }

        for key in to_remove {
            self.objects.remove(&key);
        }

        drop(cam);
        drop(settings);

        EDITOR_STATE.with(|editor_state| {
            let mut state = editor_state.borrow_mut();

            if let Some(dialog) = state.dialog.as_mut() {
                let event = dialog.update(rl);

                if event != "".to_string() {
                    state.dialog = None;

                    match event.as_str() {
                        "file.open" => {
                            if let Some(save) = SaveFile::read() {
                                self.load_from_save(save.clone(), &mut state);
                            }
                        }
                        "file.open_save" => {
                            if !self.save_file(self.camera.borrow().clone()) {
                                return;
                            }
                            if let Some(save) = SaveFile::read() {
                                self.load_from_save(save.clone(), &mut state);
                            }
                        }
                        "file.new" => {
                            state.connections.clear();
                            state.node_names.clear();
                            state.project_name = "untitled".to_string();
                            self.objects.retain(|name, _| name == "grid");
                            let mut cam = self.camera.borrow_mut();
                            cam.target = Vector2::zero();
                            cam.zoom = 1.0;
                        }
                        "file.new_save" => {
                            if !self.save_file_with_state(&mut state, self.camera.borrow().clone())
                            {
                                return;
                            }
                            state.connections.clear();
                            state.node_names.clear();
                            state.project_name = "untitled".to_string();
                            self.objects.retain(|name, _| name == "grid");
                            let mut cam = self.camera.borrow_mut();
                            cam.target = Vector2::zero();
                            cam.zoom = 1.0;
                        }
                        _ => {}
                    }
                }
            }
        });
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
        if let Some(tool_bar) = &self.tool_bar {
            tool_bar.draw(&mut d, &self.camera.borrow());
        }

        EDITOR_STATE.with(|state| {
            let mut state = state.borrow_mut();

            if let Some(dialog) = &mut state.dialog {
                dialog.draw(&mut d, &self.camera.borrow());
            }
        });
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

    fn point_in_bezier_line(p1: Vector2, p2: Vector2, point: Vector2, line_thickness: f32) -> bool {
        fn dist(a: &Vector2, b: &Vector2) -> f32 {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            (dx * dx + dy * dy).sqrt()
        }

        fn point_segment_distance(a: &Vector2, b: &Vector2, p: &Vector2) -> f32 {
            let ab = b.clone() - a.clone();
            let ap = p.clone() - a.clone();
            let ab_len2 = ab.x * ab.x + ab.y * ab.y;
            if ab_len2 == 0.0 {
                return dist(a, p);
            }
            let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len2).clamp(0.0, 1.0);
            let proj = Vector2::new(a.x + ab.x * t, a.y + ab.y * t, None);
            dist(&proj, p)
        }

        if (p1.x - p2.x).abs() < std::f32::EPSILON && (p1.y - p2.y).abs() < std::f32::EPSILON {
            return dist(&p1, &point) <= line_thickness;
        }

        let delta = p2.clone() - p1.clone();
        let length = (delta.x * delta.x + delta.y * delta.y)
            .sqrt()
            .max(std::f32::EPSILON);
        let perp = Vector2::new(-delta.y / length, delta.x / length, None);
        let midpoint = Vector2::new((p1.x + p2.x) * 0.5, (p1.y + p2.y) * 0.5, None);
        let control = midpoint + perp * (length * 0.25);

        let segments = 64usize;
        let mut prev = p1.clone();
        for i in 1..=segments {
            let t = (i as f32) / (segments as f32);
            let omt = 1.0 - t;
            let bx = omt * omt * p1.x + 2.0 * omt * t * control.x + t * t * p2.x;
            let by = omt * omt * p1.y + 2.0 * omt * t * control.y + t * t * p2.y;
            let cur = Vector2::new(bx, by, None);

            if point_segment_distance(&prev, &cur, &point) <= line_thickness {
                return true;
            }
            prev = cur;
        }

        false
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

    fn get_grid(&self) -> Rc<RefCell<Grid>> {
        let settings = self.settings.borrow();

        let grid = Rc::new(RefCell::new(Grid {
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
        }));

        {
            let mut grid_borrow = grid.borrow_mut();
            let colorscheme = self.color_schemes.borrow();
            let scheme = &self.settings.borrow().scheme;

            grid_borrow.set_property(
                "background_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_background")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
            grid_borrow.set_property(
                "square_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_square")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
            grid_borrow.set_property(
                "big_square_color".into(),
                Box::new(
                    colorscheme
                        .get_color(&scheme, "grid_big_square")
                        .unwrap_or(Color::MAGENTA),
                ),
            );
        }

        grid
    }

    fn remove_connection(name: &str, state: &mut EditorState) {
        if let Some(conn) = state.connections.remove(name) {
            conn.to.borrow_mut().write(Python::attach(|py| py.None()));
        }
    }

    fn save_file(&self, cam: Camera) -> bool {
        EDITOR_STATE.with(|state| self.save_file_with_state(&mut state.borrow_mut(), cam))
    }

    fn save_file_with_state(&self, state: &mut EditorState, cam: Camera) -> bool {
        let nodes: Vec<NodeSave> = self
            .objects
            .iter()
            .filter_map(|(_, obj)| {
                let obj = obj.borrow();
                let id = *obj
                    .get_property("id".to_string())
                    .downcast::<String>()
                    .ok()?;
                let type_name = *obj
                    .get_property("type_name".to_string())
                    .downcast::<String>()
                    .ok()?;
                let position = *obj
                    .get_property("position".to_string())
                    .downcast::<crate::structs::Vector2>()
                    .ok()?;
                Some(NodeSave {
                    id,
                    type_name,
                    position: position.into(),
                })
            })
            .collect();

        state.save_file = Some(SaveFile::from(
            state.project_name.clone(),
            nodes,
            state.connections.keys().cloned().collect(),
            CameraSave {
                position: cam.target.clone().into(),
                zoom: cam.zoom,
            },
        ));

        drop(cam);

        if let Some(save) = &state.save_file {
            save.write()
        } else {
            false
        }
    }

    fn open_file(&self) {
        EDITOR_STATE.with(|state| {
            let mut state = state.borrow_mut();

            state.dialog = Some(Dialog::new(
                "file.open".to_string(),
                vec![
                    (DialogButton::Yes, "file.open_save".to_string()),
                    (DialogButton::No, "file.open".to_string()),
                    (DialogButton::Cancel, "".to_string()),
                ],
                self.color_schemes.clone(),
                self.settings.clone(),
                self.translations.clone(),
                self.active_font.clone().unwrap(),
            ))
        })
    }

    pub fn load_from_save(&mut self, save: SaveFile, state: &mut EditorState) {
        self.objects.retain(|key, _| key == "grid");

        let lib = self.lib.clone();
        let active_font = self.active_font.clone().unwrap();
        let translations = self.translations.clone();
        let color_schemes = self.color_schemes.clone();
        let settings = self.settings.clone();

        for n in save.nodes.into_iter() {
            if let Some(node) = lib.borrow().generate(
                &n.type_name.clone(),
                active_font.clone(),
                translations.clone(),
                color_schemes.clone(),
                settings.clone(),
                n.id.clone(),
            ) {
                node.borrow_mut().position = n.position.into();

                self.objects
                    .insert(n.id.clone(), node.clone() as Rc<RefCell<dyn Object>>);

                let type_name = n.type_name.clone();
                let list = state
                    .node_names
                    .entry(type_name.clone())
                    .or_insert_with(Vec::new);
                let index_str = n.id.trim_start_matches(&type_name.clone());
                if let Ok(index) = index_str.parse::<usize>() {
                    list.push(index);
                }
            }
        }

        state.connections.clear();

        for conn_name in save.connections.clone() {
            println!("{}", conn_name.clone());
            if let Some(((from_node, from_port), (to_node, to_port))) =
                Self::parse_connection(&conn_name)
            {
                if let (Some(from), Some(to)) = (
                    self.find_port(&from_node, &from_port),
                    self.find_port(&to_node, &to_port),
                ) {
                    state
                        .connections
                        .insert(conn_name, Connection { from, to, z: 1 });
                }
            }
        }

        let mut cam = self.camera.borrow_mut();

        cam.target = save.camera.position.into();
        cam.zoom = save.camera.zoom;
    }

    pub fn find_port(
        &self,
        node_id: &str,
        port_label: &str,
    ) -> Option<Rc<RefCell<Box<crate::node::Port>>>> {
        for (id, obj) in &self.objects {
            if id.clone() == node_id.to_string() {
                let node_ref = obj.borrow();
                if let Some(node) = node_ref.as_any().downcast_ref::<Node>() {
                    for (label, _, _, port) in &node.ports {
                        if label == port_label {
                            return Some(port.clone());
                        }
                    }
                } else {
                }
            }
        }
        None
    }

    pub fn parse_connection(s: &str) -> Option<((String, String), (String, String))> {
        let mut parts = s.split("))((");

        let left = parts.next()?;
        let right = parts.next()?;

        let left = left.trim_start_matches("((");
        let right = right.trim_end_matches("))");

        fn parse_side(side: &str) -> Option<(String, String)> {
            let parts: Vec<&str> = side.splitn(2, "):(").collect();
            if parts.len() != 2 {
                return None;
            }
            let node = parts[0].to_string();
            let port = parts[1].trim_matches('"').to_string();
            Some((node, port))
        }

        let left_tuple = parse_side(left)?;
        let right_tuple = parse_side(right)?;

        Some((left_tuple, right_tuple))
    }

    fn new_file(&self) {
        EDITOR_STATE.with(|state| {
            let mut state = state.borrow_mut();

            state.dialog = Some(Dialog::new(
                "file.new".to_string(),
                vec![
                    (DialogButton::Yes, "file.new_save".to_string()),
                    (DialogButton::No, "file.new".to_string()),
                    (DialogButton::Cancel, "".to_string()),
                ],
                self.color_schemes.clone(),
                self.settings.clone(),
                self.translations.clone(),
                self.active_font.clone().unwrap(),
            ))
        })
    }
}

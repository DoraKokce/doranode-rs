use raylib::prelude::*;
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    colorscheme::ColorSchemes,
    node_libary::NodeLibary,
    objects::{Camera, Object},
    settings::Settings,
    structs::Vector2,
    translations::Translations,
    window::EDITOR_STATE,
};

pub struct NodeSelector {
    pub libary: Rc<RefCell<NodeLibary>>,
    pub colorscheme: Rc<RefCell<ColorSchemes>>,
    pub settings: Rc<RefCell<Settings>>,
    pub translations: Rc<RefCell<Translations>>,
    pub font: Rc<RefCell<Font>>,
    pub size: Vector2,
    pub prefix_expanded: RefCell<HashMap<String, bool>>,
    pub module_expanded: RefCell<HashMap<String, bool>>,
}

impl Object for NodeSelector {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw(&self, d: &mut RaylibDrawHandle, _camera: &Camera) {
        d.draw_rectangle(
            0,
            0,
            self.size.x as i32,
            self.size.y as i32,
            self.colorscheme
                .borrow()
                .get_color(&self.settings.borrow().scheme, "node_background")
                .unwrap(),
        );

        let hierarchy = self.libary.borrow().get_hierarchy();
        let mut y = 10;

        for (prefix, modules) in &hierarchy {
            let p_exp = *self.prefix_expanded.borrow().get(prefix).unwrap_or(&false);

            d.draw_text_ex(
                &*self.font.borrow(),
                &format!("{} {}", if p_exp { "v" } else { ">" }, prefix),
                Vector2::new(10.0, y as f32, None),
                22.0,
                1.0,
                self.colorscheme
                    .borrow()
                    .get_color(&self.settings.borrow().scheme, "node_foreground")
                    .unwrap(),
            );
            y += 26;

            if !p_exp {
                continue;
            }

            for (module, nodes) in modules {
                let key = format!("{prefix}:{module}");
                let translation = self
                    .translations
                    .borrow()
                    .get_node_translation(&self.settings.borrow().language, &key)
                    .title;

                if nodes.is_empty() {
                    d.draw_text_ex(
                        &*self.font.borrow(),
                        &format!("    - {}", translation),
                        Vector2::new(20.0, y as f32, None),
                        20.0,
                        1.0,
                        self.colorscheme
                            .borrow()
                            .get_color(&self.settings.borrow().scheme, "node_foreground")
                            .unwrap(),
                    );
                    y += 22;
                    continue;
                }

                let m_exp = *self.module_expanded.borrow().get(&key).unwrap_or(&false);
                d.draw_text_ex(
                    &*self.font.borrow(),
                    &format!("    {} {}", if m_exp { "v" } else { ">" }, translation),
                    Vector2::new(20.0, y as f32, None),
                    20.0,
                    1.0,
                    self.colorscheme
                        .borrow()
                        .get_color(&self.settings.borrow().scheme, "node_foreground")
                        .unwrap(),
                );
                y += 22;

                if !m_exp {
                    continue;
                }

                for node in nodes {
                    let full_key = format!("{prefix}:{module}.{node}");
                    let translated = self
                        .translations
                        .borrow()
                        .get_node_translation(&self.settings.borrow().language, &full_key)
                        .title;

                    d.draw_text_ex(
                        &*self.font.borrow(),
                        &format!("        - {}", translated),
                        Vector2::new(30.0, y as f32, None),
                        18.0,
                        1.0,
                        self.colorscheme
                            .borrow()
                            .get_color(&self.settings.borrow().scheme, "node_foreground")
                            .unwrap(),
                    );
                    y += 20;
                }
            }
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, _t: &RaylibThread, _c: &Camera) {
        self.size = Vector2::new(200.0, rl.get_screen_height() as f32, None);
        let mouse = rl.get_mouse_position();
        let hierarchy = self.libary.borrow().get_hierarchy();
        let mut y = 10;
        let mut prefix_width: f32 = 0.0;
        let mut module_width: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for (prefix, modules) in &hierarchy {
            let text_size = self
                .font
                .borrow()
                .measure_text(&format!("> {}", prefix), 22.0, 1.0);
            let prefix_rect = Rectangle {
                x: 10.0,
                y: y as f32,
                width: text_size.x,
                height: 22.0,
            };
            max_width = max_width.max(text_size.x + 20.0);
            prefix_width = text_size.x;

            if prefix_rect.check_collision_point_rec(mouse)
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                let old = *self.prefix_expanded.borrow().get(prefix).unwrap_or(&false);
                self.prefix_expanded
                    .borrow_mut()
                    .insert(prefix.clone(), !old);
            }

            y += 26;

            if !self
                .prefix_expanded
                .borrow()
                .get(prefix)
                .copied()
                .unwrap_or(false)
            {
                continue;
            }

            for (module, nodes) in modules {
                let key = format!("{prefix}:{module}");
                let translation = self
                    .translations
                    .borrow()
                    .get_node_translation(&self.settings.borrow().language, &format!("{key}"))
                    .title;
                let text_size =
                    self.font
                        .borrow()
                        .measure_text(&format!("> {}", translation), 20.0, 1.0);
                max_width = max_width.max(text_size.x + prefix_width - 20.0);
                module_width = text_size.x + prefix_width - 20.0;

                if nodes.is_empty() {
                    let module_rect = Rectangle {
                        x: 20.0,
                        y: y as f32,
                        width: text_size.x,
                        height: 22.0,
                    };
                    if module_rect.check_collision_point_rec(mouse)
                        && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        EDITOR_STATE
                            .with(|state| state.borrow_mut().selected_module = Some(key.clone()));
                    }
                    y += 22;
                    continue;
                }

                let module_rect = Rectangle {
                    x: 20.0,
                    y: y as f32,
                    width: text_size.x,
                    height: 22.0,
                };
                if module_rect.check_collision_point_rec(mouse)
                    && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    let old = *self.module_expanded.borrow().get(&key).unwrap_or(&false);
                    self.module_expanded.borrow_mut().insert(key.clone(), !old);
                }

                y += 22;

                if !self
                    .module_expanded
                    .borrow()
                    .get(&key)
                    .copied()
                    .unwrap_or(false)
                {
                    continue;
                }

                for node in nodes {
                    let translation = self
                        .translations
                        .borrow()
                        .get_node_translation(
                            &self.settings.borrow().language,
                            &format!("{key}.{node}"),
                        )
                        .title;
                    let text_size = self.font.borrow().measure_text(&translation, 20.0, 1.0);
                    let node_rect = Rectangle {
                        x: 75.0,
                        y: y as f32,
                        width: text_size.x,
                        height: 20.0,
                    };
                    max_width = max_width.max(text_size.x + module_width - 90.0);
                    if node_rect.check_collision_point_rec(mouse)
                        && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        let full_key = format!("{prefix}:{module}.{node}");
                        EDITOR_STATE
                            .with(|state| state.borrow_mut().selected_module = Some(full_key));
                    }
                    y += 20;
                }
            }
        }

        self.size.x = max_width;
        EDITOR_STATE.with(|state| {
            state.borrow_mut().selector_size = self.size.clone();
        });
    }

    fn get_property(&self, _: String) -> Box<dyn Any + 'static> {
        Box::new(())
    }

    fn set_property(&mut self, _: String, _: Box<dyn Any>) {}
}

impl NodeSelector {
    pub fn new(
        libary: Rc<RefCell<NodeLibary>>,
        font: Rc<RefCell<Font>>,
        colorscheme: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        translations: Rc<RefCell<Translations>>,
    ) -> Self {
        Self {
            libary,
            font,
            colorscheme,
            settings,
            translations,
            size: Vector2::zero(),
            prefix_expanded: RefCell::new(HashMap::new()),
            module_expanded: RefCell::new(HashMap::new()),
        }
    }
}

pub struct ToolBarItem {
    pub label: String,
    pub on_click: Option<String>,
    pub children: Vec<ToolBarItem>,
    pub expanded: bool,
}

pub struct ToolBar {
    pub position: Vector2,
    pub colorscheme: Rc<RefCell<ColorSchemes>>,
    pub translations: Rc<RefCell<Translations>>,
    pub settings: Rc<RefCell<Settings>>,
    pub font: Rc<RefCell<Font>>,
    pub items: Vec<ToolBarItem>,
    pub events: Vec<String>,
}

impl Object for ToolBar {
    fn draw(&self, d: &mut RaylibDrawHandle, _camera: &Camera) {
        let screen_width = d.get_screen_width() as f32;
        d.draw_rectangle(
            self.position.x as i32,
            0,
            screen_width as i32 - self.position.x as i32,
            40,
            self.colorscheme
                .borrow()
                .get_color(&self.settings.borrow().scheme, "topbar_background")
                .unwrap(),
        );

        let mut x_offset = 10.0 + self.position.x;
        for item in &self.items {
            let translation = self.translations.borrow().get_gui_translation(
                &self.settings.borrow().language,
                &format!("toolbar.{}", item.label),
            );
            d.draw_rectangle(
                x_offset as i32 - 5,
                5,
                (self.font.borrow().measure_text(&translation, 20.0, 1.0).x + 10.0) as i32,
                30,
                self.colorscheme
                    .borrow()
                    .get_color(
                        &self.settings.borrow().scheme,
                        if item.expanded {
                            "topbar_button_background_expanded"
                        } else {
                            "topbar_button_background"
                        },
                    )
                    .unwrap(),
            );
            d.draw_text_ex(
                &*self.font.borrow(),
                &translation,
                Vector2::new(x_offset, 10.0, None),
                20.0,
                1.0,
                self.colorscheme
                    .borrow()
                    .get_color(&self.settings.borrow().scheme, "topbar_foreground")
                    .unwrap(),
            );

            if item.expanded {
                let mut y_offset = 40.0;
                d.draw_rectangle(
                    x_offset as i32,
                    y_offset as i32,
                    self.get_largest_width(item) as i32 + 20,
                    30 * item.children.len() as i32,
                    self.colorscheme
                        .borrow()
                        .get_color(&self.settings.borrow().scheme, "topbar_button_background")
                        .unwrap(),
                );
                for child in &item.children {
                    let translation = self.translations.borrow().get_gui_translation(
                        &self.settings.borrow().language,
                        &format!("toolbar.{}.{}", item.label, child.label),
                    );
                    d.draw_text_ex(
                        &*self.font.borrow(),
                        &translation,
                        Vector2::new(x_offset + 10.0, y_offset, None),
                        20.0,
                        1.0,
                        self.colorscheme
                            .borrow()
                            .get_color(&self.settings.borrow().scheme, "topbar_foreground")
                            .unwrap(),
                    );
                    y_offset += 30.0;
                }
            }

            x_offset += self.font.borrow().measure_text(&item.label, 20.0, 1.0).x + 20.0;
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, _camera: &Camera) {
        let mouse_pos = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mut x_offset = 10.0;

            for item in &mut self.items {
                let translation = self.translations.borrow().get_gui_translation(
                    &self.settings.borrow().language,
                    &format!("toolbar.{}", item.label),
                );
                let text_width = self.font.borrow().measure_text(&translation, 20.0, 1.0).x;
                let text_rect = Rectangle::new(
                    x_offset + self.position.x - 5.0,
                    0.0,
                    text_width + 10.0,
                    40.0,
                );

                if text_rect.check_collision_point_rec(mouse_pos.clone()) {
                    if !item.children.is_empty() {
                        item.expanded = !item.expanded;
                    }
                    if let Some(event) = &item.on_click {
                        self.events.push(event.clone());
                    }
                }

                if item.expanded {
                    let mut y_offset = 40.0;
                    for child in &mut item.children {
                        let translation = self.translations.borrow().get_gui_translation(
                            &self.settings.borrow().language,
                            &format!("toolbar.{}.{}", item.label, child.label),
                        );
                        let child_rect = Rectangle::new(
                            x_offset + self.position.x,
                            y_offset + self.position.y,
                            self.font.borrow().measure_text(&translation, 20.0, 1.0).x + 20.0,
                            30.0,
                        );
                        if child_rect.check_collision_point_rec(mouse_pos.clone()) {
                            if let Some(event) = &child.on_click {
                                self.events.push(event.clone());
                            }
                            item.expanded = false;
                        }
                        y_offset += 30.0;
                    }
                }

                x_offset += text_width + 20.0;
            }
        }
    }

    fn set_property(&mut self, _key: String, _value: Box<dyn Any>) {}
    fn get_property(&self, _key: String) -> Box<dyn Any> {
        Box::new(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ToolBar {
    pub fn new(
        colorscheme: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        font: Rc<RefCell<Font>>,
        translations: Rc<RefCell<Translations>>,
    ) -> Self {
        Self {
            position: Vector2::zero(),
            colorscheme,
            settings,
            font,
            translations,
            items: vec![],
            events: vec![],
        }
    }

    pub fn add_item(&mut self, item: ToolBarItem) {
        self.items.push(item);
    }

    fn get_largest_width(&self, item: &ToolBarItem) -> f32 {
        let mut max_x: f32 = 0.0;
        for child in &item.children {
            let translation = self.translations.borrow().get_gui_translation(
                &self.settings.borrow().language,
                &format!("toolbar.{}.{}", item.label, child.label),
            );
            max_x = max_x.max(self.font.borrow().measure_text(&translation, 20.0, 1.0).x);
        }
        max_x
    }
}

pub fn draw_text_wordwrap(
    d: &mut RaylibDrawHandle,
    text: &str,
    x: i32,
    y: i32,
    max_width: i32,
    font: &Font,
    font_size: i32,
    color: Color,
    line_spacing: i32,
) {
    let mut cur_y = y;
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = String::new();

    for (i, word) in words.iter().enumerate() {
        let test = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };

        let width = d.measure_text(&test, font_size);

        if width > max_width {
            d.draw_text_ex(
                font,
                &line,
                Vector2::new(x as f32, cur_y as f32, None),
                font_size as f32,
                1.0,
                color,
            );

            cur_y += font_size + line_spacing;
            line = word.to_string();
        } else {
            line = test;
        }

        if i == words.len() - 1 {
            d.draw_text_ex(
                font,
                &line,
                Vector2::new(x as f32, cur_y as f32, None),
                font_size as f32,
                1.0,
                color,
            );
        }
    }
}

pub enum DialogButton {
    Ok,
    Cancel,
    Yes,
    No,
}

pub struct Dialog {
    pub type_name: String,
    pub buttons: Vec<(DialogButton, String)>,
    pub colorscheme: Rc<RefCell<ColorSchemes>>,
    pub translations: Rc<RefCell<Translations>>,
    pub font: Rc<RefCell<Font>>,
    pub settings: Rc<RefCell<Settings>>,
}

impl Dialog {
    pub fn draw(&self, d: &mut RaylibDrawHandle, _camera: &Camera) {
        let pos = Vector2::new(
            d.get_screen_width() as f32 / 4.0,
            d.get_screen_height() as f32 / 4.0,
            None,
        );
        let size = Vector2::new(
            d.get_screen_width() as f32 / 2.0,
            d.get_screen_height() as f32 / 2.0,
            None,
        );

        let rect = Rectangle::new(pos.x, pos.y - 40.0, size.x, size.y + 80.0);
        let roundness = 0.1;
        let segments = 10;
        let bckgcolor = self
            .colorscheme
            .borrow()
            .get_color(&self.settings.borrow().scheme, "dialog_background")
            .unwrap();
        let brdcolor = self
            .colorscheme
            .borrow()
            .get_color(&self.settings.borrow().scheme, "dialog_border")
            .unwrap();
        let frgcolor = self
            .colorscheme
            .borrow()
            .get_color(&self.settings.borrow().scheme, "dialog_foreground")
            .unwrap();

        d.draw_rectangle_rounded(rect, roundness, segments, bckgcolor);
        d.draw_rectangle_rounded_lines_ex(rect, roundness, segments, 10.0, brdcolor);
        d.draw_line_ex(
            Vector2::new(pos.x, pos.y + 20.0, None),
            Vector2::new(pos.x + size.x, pos.y + 20.0, None),
            10.0,
            brdcolor,
        );
        let title = self.translations.borrow().get_gui_translation(
            &self.settings.borrow().language,
            &format!("dialog.{}.title", self.type_name),
        );
        let title_width = self.font.borrow().measure_text(&title, 30.0, 1.0).x;
        d.draw_text_ex(
            &*self.font.borrow(),
            &title,
            Vector2::new(pos.x + size.x / 2.0 - title_width / 2.0, pos.y - 35.0, None),
            30.0,
            1.0,
            frgcolor,
        );
        let content = self.translations.borrow().get_gui_translation(
            &self.settings.borrow().language,
            &format!("dialog.{}.content", self.type_name),
        );
        draw_text_wordwrap(
            d,
            &content,
            pos.x as i32 + 20,
            pos.y as i32 + 40,
            size.x as i32 - 40,
            &*self.font.borrow(),
            25,
            frgcolor,
            5,
        );

        let button_width = 100.0;
        let button_height = 40.0;
        let buttons_total_width = self.buttons.len() as f32 * (button_width + 10.0);
        let mut button_x = pos.x + size.x / 2.0 - buttons_total_width / 2.0;
        let button_y = pos.y + size.y - 60.0;

        for (button_type, _) in &self.buttons {
            let button_label = match button_type {
                DialogButton::Ok => "text.ok",
                DialogButton::Cancel => "text.cancel",
                DialogButton::Yes => "text.yes",
                DialogButton::No => "text.no",
            };
            let translation = self.translations.borrow().get_gui_translation(
                &self.settings.borrow().language,
                &format!("{}", button_label.to_lowercase()),
            );

            let button_rect = Rectangle::new(button_x, button_y, button_width, button_height);
            d.draw_rectangle_rounded(button_rect, 0.1, 10, brdcolor);

            let label_width = self.font.borrow().measure_text(&translation, 16.0, 1.0).x;
            d.draw_text_ex(
                &*self.font.borrow(),
                &translation,
                Vector2::new(
                    button_x + button_width / 2.0 - label_width / 2.0,
                    button_y + 12.0,
                    None,
                ),
                16.0,
                1.0,
                frgcolor,
            );

            button_x += button_width + 10.0;
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) -> String {
        let mouse_pos = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let pos = Vector2::new(
                rl.get_screen_width() as f32 / 4.0,
                rl.get_screen_height() as f32 / 4.0,
                None,
            );
            let size = Vector2::new(
                rl.get_screen_width() as f32 / 2.0,
                rl.get_screen_height() as f32 / 2.0,
                None,
            );

            let button_width = 100.0;
            let button_height = 40.0;
            let buttons_total_width = self.buttons.len() as f32 * (button_width + 10.0);
            let mut button_x = pos.x + size.x / 2.0 - buttons_total_width / 2.0;
            let button_y = pos.y + size.y - 60.0;

            let mut event = "".to_string();

            for i in 0..self.buttons.len() {
                let button_rect = Rectangle::new(button_x, button_y, button_width, button_height);
                if button_rect.check_collision_point_rec(mouse_pos) {
                    if let Some((_, e)) = self.buttons.get(i) {
                        event = e.clone();
                    }
                }
                button_x += button_width + 10.0;
            }

            event
        } else {
            "".to_string()
        }
    }

    fn get_property(&self, _key: String) -> Box<dyn Any + 'static> {
        Box::new(())
    }

    fn set_property(&mut self, _key: String, _value: Box<dyn Any>) {}
}

impl Dialog {
    pub fn new(
        type_name: String,
        buttons: Vec<(DialogButton, String)>,
        colorscheme: Rc<RefCell<ColorSchemes>>,
        settings: Rc<RefCell<Settings>>,
        translations: Rc<RefCell<Translations>>,
        font: Rc<RefCell<Font>>,
    ) -> Self {
        Self {
            type_name,
            buttons,
            colorscheme,
            settings,
            translations,
            font,
        }
    }
}

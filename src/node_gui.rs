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

    fn draw(&self, d: &mut RaylibDrawHandle, camera: &Camera) {
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
                let m_exp = *self.module_expanded.borrow().get(&key).unwrap_or(&false);

                let translation = self
                    .translations
                    .borrow()
                    .get_node_translation(&self.settings.borrow().language, &key)
                    .title;

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

        for (prefix, modules) in &hierarchy {
            let prefix_rect = Rectangle {
                x: 10.0,
                y: y as f32,
                width: 160.0,
                height: 22.0,
            };

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
                let module_rect = Rectangle {
                    x: 20.0,
                    y: y as f32,
                    width: 160.0,
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
                    let node_rect = Rectangle {
                        x: 30.0,
                        y: y as f32,
                        width: 160.0,
                        height: 20.0,
                    };

                    if node_rect.check_collision_point_rec(mouse)
                        && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        EDITOR_STATE.with(|state| {
                            state.borrow_mut().selected_module =
                                Some(format!("{prefix}:{module}.{node}"))
                        });
                    }

                    y += 20;
                }
            }
        }
    }

    fn get_property(&self, key: String) -> Box<dyn Any + 'static> {
        Box::new(())
    }

    fn set_property(&mut self, key: String, value: Box<dyn Any>) {}
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

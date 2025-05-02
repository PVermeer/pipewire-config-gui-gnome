use super::NavPage;
use crate::application::{
    Application,
    pipewire::pipewire::Pipewire,
    shared::init::{Init, InitTrait},
};
use libadwaita::{
    NavigationPage,
    gtk::{self, Label, prelude::BoxExt},
};
use serde_json::Value;
use std::{collections::HashMap, rc::Rc};

pub struct SurroundPage {
    pub page: NavigationPage,
    content_box: gtk::Box,
    init: Init,
}
impl NavPage for SurroundPage {
    const LABEL: &str = "surround-page";

    fn new() -> Self {
        let (page, _header, content_box, init) = Self::build_nav_page("Surround");

        return Self {
            page,
            content_box,
            init,
        };
    }

    fn is_init(&self) -> bool {
        self.init.get_state()
    }

    fn init(&mut self, application: Rc<Application>) {
        self.on_onit(application);
        self.init.set_state(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.page
    }
}
impl SurroundPage {
    fn on_onit(&self, application: Rc<Application>) {
        let pipewire = application.pipewire.clone();

        let labels = self.build_default_sections(pipewire);

        for label in labels {
            self.content_box.append(&label);
        }
    }

    fn build_default_sections(&self, pipewire: Rc<Pipewire>) -> Vec<Label> {
        let default = &pipewire.surround.default;
        let mut map: HashMap<Option<&str>, Vec<(&str, &Value)>> = HashMap::new();

        for (key, value) in default {
            let mut section = None;
            let mut prop: &str = key;

            if key.contains('.') {
                let mut split = key.split('.');
                section = split.next();
                prop = split.next().unwrap();
            }

            let mapped_section = match map.get_mut(&section) {
                None => {
                    map.insert(section, Vec::new());
                    map.get_mut(&section).unwrap()
                }
                Some(value) => value,
            };
            mapped_section.push((prop, value));
        }

        let mut labels: Vec<Label> = Vec::new();

        for (section_name, values) in map {
            let label_markup = match section_name {
                None => format!("<b></b>{:?}", values),
                Some(value) => format!("<b>{}</b>{:?}", value, values),
            };

            let label = Label::builder()
                .label(label_markup)
                .wrap(true)
                .use_markup(true)
                .halign(gtk::Align::Start)
                .build();

            labels.push(label);
        }
        labels
    }
}

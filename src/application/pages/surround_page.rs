use super::NavPage;
use crate::application::{
    Application,
    pipewire::pipewire::Pipewire,
    shared::init::{Init, InitTrait},
};
use convert_case::{Case, Casing};
use libadwaita::{
    NavigationPage, SpinRow, SwitchRow,
    glib::object::Cast,
    gtk::{self, Adjustment, Label, Widget, prelude::BoxExt},
};
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

        self.build_default_sections(pipewire);
        // let labels = self.build_default_sections(pipewire);

        // for label in labels {
        //     self.content_box.append(&label);
        // }
    }

    fn build_default_sections(&self, pipewire: Rc<Pipewire>) {
        let default = &pipewire.surround.default;
        let mut map: HashMap<Option<&str>, Vec<(&str, &serde_json::Value)>> = HashMap::new();

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

        // let mut labels: Vec<Label> = Vec::new();

        for (section_name, values) in map {
            let label_markup = match section_name {
                None => format!("<b>Misc</b>"),
                Some(section) => format!("<b>{}</b>", section.to_case(Case::Title)),
            };

            let label = Label::builder()
                .label(label_markup)
                .wrap(true)
                .use_markup(true)
                .halign(gtk::Align::Start)
                .build();

            self.content_box.append(&label);
            // labels.push(label);
            for (key, value) in values {
                self.content_box.append(&self.build_input_row(key, value));
            }
        }
        // labels
    }

    fn build_input_row(&self, key: &str, value: &serde_json::Value) -> Widget {
        let title = key.from_case(Case::Kebab).to_case(Case::Sentence);

        match value {
            serde_json::Value::Bool(value) => SwitchRow::builder()
                .title(title)
                .active(value.to_owned())
                .build()
                .upcast(),

            // TODO: maybe slider? (ActionRow->Scale)
            serde_json::Value::Number(value) => SpinRow::builder()
                .title(title)
                .adjustment(
                    &Adjustment::builder()
                        .lower(f64::MIN)
                        .upper(f64::MAX)
                        .page_increment(1.0)
                        .step_increment(1.0)
                        .build(),
                )
                .digits(1)
                .editable(true)
                .value(value.as_f64().unwrap())
                .build()
                .upcast(),

            // serde_json::Value::String(_value) => Row::None,
            // serde_json::Value::Array(_value) => Row::None,
            // serde_json::Value::Object(_value) => Row::None,
            // serde_json::Value::Null => Row::None,
            _ => SwitchRow::builder().title(title).build().upcast(),
        }
    }
}

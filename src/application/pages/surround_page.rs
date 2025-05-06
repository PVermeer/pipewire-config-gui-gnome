use super::NavPage;
use crate::application::{
    Application,
    pipewire::pipewire::Pipewire,
    shared::init::{Init, InitTrait},
};
use convert_case::{Case, Casing};
use libadwaita::{
    ComboRow, EntryRow, NavigationPage, PreferencesGroup, PreferencesPage, SpinRow, SwitchRow,
    glib::object::Cast,
    gtk::{Adjustment, StringList, Widget},
    prelude::{PreferencesGroupExt, PreferencesPageExt},
};
use std::{collections::HashMap, rc::Rc};

pub struct SurroundPage {
    pub nav_page: NavigationPage,
    pref_page: PreferencesPage,
    init: Init,
}
impl NavPage for SurroundPage {
    const LABEL: &str = "surround-page";

    fn new() -> Self {
        let (nav_page, pref_page, _header, init) = Self::build_pref_page("Surround");

        return Self {
            nav_page,
            pref_page,
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
        &self.nav_page
    }
}
impl SurroundPage {
    fn on_onit(&self, application: Rc<Application>) {
        let pipewire = application.pipewire.clone();

        let section_groups = self.build_default_sections(pipewire);
        for group in section_groups {
            self.pref_page.add(&group);
        }
    }

    fn build_default_sections(&self, pipewire: Rc<Pipewire>) -> Vec<PreferencesGroup> {
        let default = &pipewire.surround.default;
        let mut map: HashMap<Option<&str>, Vec<(&str, &(serde_json::Value, Option<Vec<String>>))>> =
            HashMap::new();
        let mut preferences_groups: Vec<PreferencesGroup> = Vec::new();

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

        for (section_name, values) in map {
            let section = match section_name {
                None => String::new(),
                Some(section) => section.to_case(Case::Title),
            };
            let preferences_group = PreferencesGroup::builder().title(section).build();

            for (key, (value, options)) in values {
                let input_row = self.build_input_row(key, value, options);
                preferences_group.add(&input_row);
            }
            preferences_groups.push(preferences_group);
        }
        preferences_groups
    }

    fn build_input_row(
        &self,
        key: &str,
        value: &serde_json::Value,
        options: &Option<Vec<String>>,
    ) -> Widget {
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

            serde_json::Value::String(value) => match options {
                Some(options_value) => {
                    let list = StringList::new(&[]);
                    for option in options_value {
                        list.append(&option);
                    }
                    let combo_row = ComboRow::builder().title(title).model(&list).build();
                    combo_row.upcast()
                }
                None => EntryRow::builder()
                    .title(title)
                    .text(value)
                    .build()
                    .upcast(),
            },

            // serde_json::Value::Array(_value) => Row::None,
            // serde_json::Value::Object(_value) => Row::None,
            // serde_json::Value::Null => Row::None,
            _ => SwitchRow::builder().title(title).build().upcast(),
        }
    }
}

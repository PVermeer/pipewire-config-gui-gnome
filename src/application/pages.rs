mod main_page;
mod surround_page;

use super::{
    Application,
    pipewire::config::PwConfig,
    shared::init::{Init, InitTrait},
};
use convert_case::{Case, Casing};
use libadwaita::{
    ComboRow, EntryRow, HeaderBar, NavigationPage, NavigationSplitView, PreferencesGroup,
    PreferencesPage, SpinRow, SwitchRow, ToolbarView,
    gio::{ActionEntry, SimpleActionGroup, prelude::ActionMapExtManual},
    glib::{VariantTy, object::Cast, variant::ToVariant},
    gtk::{
        self, Adjustment, Orientation, StringList, Widget,
        prelude::{EditableExt, WidgetExt},
    },
    prelude::{ComboRowExt, PreferencesGroupExt},
};
use main_page::MainPage;
use serde_json::json;
use std::{collections::HashMap, rc::Rc};
use surround_page::SurroundPage;

#[repr(i32)]
pub enum Page {
    Main,
    Surround,
}

pub struct Pages {
    pub main: MainPage,
    pub surround: SurroundPage,
}
impl Pages {
    pub fn new() -> Self {
        Self {
            main: MainPage::new(),
            surround: SurroundPage::new(),
        }
    }
}

pub trait NavPage {
    const LABEL: &str;

    fn new() -> Self;

    fn init(&mut self, _application: Rc<Application>);

    fn is_init(&self) -> bool;

    fn get_navpage(&self) -> &NavigationPage;

    fn get_title(&self) -> &str;

    fn load_page(&mut self, application: Rc<Application>, view: &NavigationSplitView) {
        if !self.is_init() {
            self.init(application);
        }

        let nav_page = self.get_navpage();
        if nav_page.parent().is_some() {
            return;
        };
        view.set_content(Some(nav_page));
    }

    fn build_nav_page(title: &str) -> (NavigationPage, HeaderBar, gtk::Box, Init) {
        const MARGIN: i32 = 20;

        let content_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(MARGIN)
            .margin_bottom(MARGIN)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .build();

        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&content_box));

        let nav_page = NavigationPage::builder()
            .title(title)
            .tag(title)
            .child(&toolbar)
            .build();

        let init = Init::new();

        return (nav_page, header, content_box, init);
    }
}

pub trait PrefPage: NavPage {
    const ACTION_LABEL: &str;
    const INPUT_ACTION_LABEL: &str;
    const INPUT_PAGE_ACTION_LABEL: &str;

    fn build_pref_page(
        title: &str,
    ) -> (
        NavigationPage,
        PreferencesPage,
        HeaderBar,
        Init,
        SimpleActionGroup,
    ) {
        let pref_page = PreferencesPage::new();
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        let action_group = SimpleActionGroup::new();
        let input_action = Self::build_input_action();

        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&pref_page));

        action_group.add_action_entries([input_action]);
        pref_page.insert_action_group(Self::ACTION_LABEL, Some(&action_group));

        let nav_page = NavigationPage::builder()
            .title(title)
            .tag(title)
            .child(&toolbar)
            .build();

        let init = Init::new();

        return (nav_page, pref_page, header, init, action_group);
    }

    fn build_page_switch(&self) -> PreferencesGroup {
        let key = format!("enable-{}", self.get_title().to_lowercase());

        let preferences_group = PreferencesGroup::builder().build();
        let enable_switch = self
            .build_input_row_for_pref_group(&key, &serde_json::Value::Bool(false), &None)
            .unwrap();
        preferences_group.add(&enable_switch);

        preferences_group
    }

    fn build_sections_from_default(&self, pw_config: &PwConfig) -> Vec<PreferencesGroup> {
        let default = &pw_config.default;
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

        for (section_name, mut values) in map {
            values.sort_by(|(a, _), (b, _)| a.cmp(b));

            let section = match section_name {
                None => String::new(),
                Some(section) => section.to_case(Case::Title),
            };
            let preferences_group = PreferencesGroup::builder().title(section).build();

            for (key, (value, options)) in values {
                if let Some(input_row) = self.build_input_row_for_pref_group(key, value, options) {
                    preferences_group.add(&input_row);
                }
            }
            preferences_groups.push(preferences_group);
        }

        preferences_groups.sort_by(|a, b| a.title().cmp(&b.title()));

        preferences_groups
    }

    fn build_input_row_for_pref_group(
        &self,
        key: &str,
        value: &serde_json::Value,
        options: &Option<Vec<String>>,
    ) -> Option<Widget> {
        let title = key
            .to_lowercase()
            .from_case(Case::Kebab)
            .to_case(Case::Sentence);

        match value {
            serde_json::Value::Bool(value) => {
                let key = key.to_owned();
                let build = SwitchRow::builder()
                    .title(title)
                    .active(value.to_owned())
                    .build();

                build.connect_active_notify(move |switch_row| {
                    let json_variant = json!({ &key: &switch_row.is_active() })
                        .to_string()
                        .to_variant();

                    switch_row
                        .activate_action(Self::INPUT_PAGE_ACTION_LABEL, Some(&json_variant))
                        .unwrap();
                });

                Some(build.upcast())
            }

            // TODO: maybe slider? (ActionRow->Scale)
            serde_json::Value::Number(value) => {
                let key = key.to_owned();
                let build = SpinRow::builder()
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
                    .build();

                build.connect_value_notify(move |spin_row| {
                    let json_variant = json!({ &key: &spin_row.value() }).to_string().to_variant();

                    spin_row
                        .activate_action(Self::INPUT_PAGE_ACTION_LABEL, Some(&json_variant))
                        .unwrap();
                });

                Some(build.upcast())
            }

            serde_json::Value::String(value) => match options {
                Some(options_value) => {
                    let list = StringList::new(&[]);
                    for option in options_value {
                        list.append(&option);
                    }

                    let key = key.to_owned();
                    let build = ComboRow::builder().title(title).model(&list).build();

                    build.connect_selected_item_notify(move |combo_row| {
                        let selected_string =
                            list.string(combo_row.selected()).unwrap().to_string();
                        let json_variant =
                            json!({ &key: &selected_string }).to_string().to_variant();

                        combo_row
                            .activate_action(Self::INPUT_PAGE_ACTION_LABEL, Some(&json_variant))
                            .unwrap();
                    });

                    Some(build.upcast())
                }
                None => {
                    let key = key.to_owned();
                    let build = EntryRow::builder().title(title).text(value).build();

                    build.connect_text_notify(move |entry_row| {
                        let input_string = entry_row.text().to_string();
                        let json_variant = json!({ &key: &input_string }).to_string().to_variant();

                        entry_row
                            .activate_action(Self::INPUT_PAGE_ACTION_LABEL, Some(&json_variant))
                            .unwrap();
                    });

                    Some(build.upcast())
                }
            },

            // serde_json::Value::Array(_value) => None,
            // serde_json::Value::Object(_value) => None,
            // serde_json::Value::Null => None,
            _ => None,
        }
    }

    fn build_input_action() -> ActionEntry<SimpleActionGroup> {
        let action = ActionEntry::builder(Self::INPUT_ACTION_LABEL)
            .parameter_type(Some(VariantTy::STRING))
            .activate(move |_group, _action, parameter| {
                let string_value = parameter.unwrap().try_get::<String>().unwrap();
                let json_value: serde_json::Value = serde_json::from_str(&string_value).unwrap();
                println!("========= INPUT ACTION:  {:#?}", json_value);
            })
            .build();

        action
    }
}

use super::{NavPage, Page, PageState, PrefPage, PreferencesPageEntries};
use crate::application::Application;
use libadwaita::{
    NavigationPage, PreferencesPage,
    gio::{SimpleActionGroup, prelude::ActionMapExtManual},
    prelude::PreferencesPageExt,
};
use std::{collections::HashMap, rc::Rc};

pub struct SurroundPage {
    pub nav_page: NavigationPage,
    pref_page: PreferencesPage,
    pref_groups: PreferencesPageEntries,
    state: PageState,
    title: String,
    actions: SimpleActionGroup,
}
impl NavPage for SurroundPage {
    const LABEL: &str = "surround-page";
    const LOG_TARGET: &str = Self::LABEL;

    fn new() -> Self {
        let title = String::from("Surround");
        let (nav_page, pref_page, _header, state, actions) = Self::build_pref_page(&title);

        return Self {
            nav_page,
            pref_page,
            pref_groups: HashMap::with_capacity(0),
            state,
            title,
            actions,
        };
    }

    fn is_init(&self) -> bool {
        self.state.get_init()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_state(&self) -> &PageState {
        &self.state
    }

    fn init(&mut self, application: Rc<Application>) {
        self.on_onit(application);
        self.state.set_init(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.nav_page
    }
}
impl PrefPage for SurroundPage {
    const ACTION_LABEL: &str = "surround";
    const INPUT_ACTION_LABEL: &str = "input";
    const INPUT_PAGE_ACTION_LABEL: &str = "surround.input";
    const PAGE_ENABLE_ACTION_LABEL: &str = "page-enable";
    const PAGE_ENABLE_PAGE_ACTION_LABEL: &str = "surround.page-enable";

    fn get_pref_groups(&self) -> &PreferencesPageEntries {
        &self.pref_groups
    }

    fn set_state_enabled(&mut self, enabled: bool) {
        self.state.set_page_enabled(enabled);
    }
}
impl SurroundPage {
    fn on_onit(&mut self, application: Rc<Application>) {
        let pipewire = application.pipewire.clone();

        let input_action = self.build_input_action(&pipewire.surround);
        let page_enabled_action = self.build_page_switch_action(application, Page::Surround);
        self.actions
            .add_action_entries([input_action, page_enabled_action]);

        let enable_pref_group = self.build_page_switch();
        self.pref_page.add(&enable_pref_group);

        self.pref_groups = self.build_sections_from_default(&pipewire.surround.borrow());
        for (group, _rows) in &self.pref_groups {
            self.pref_page.add(group);
        }

        // TODO based on current settings
        self.set_enabled(false);
    }
}

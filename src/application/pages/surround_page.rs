use super::{NavPage, PrefPage};
use crate::application::{
    Application,
    shared::init::{Init, InitTrait},
};
use libadwaita::{NavigationPage, PreferencesPage, prelude::PreferencesPageExt};
use std::rc::Rc;

pub struct SurroundPage {
    pub nav_page: NavigationPage,
    pref_page: PreferencesPage,
    init: Init,
    title: String,
}
impl NavPage for SurroundPage {
    const LABEL: &str = "surround-page";

    fn new() -> Self {
        let title = String::from("Surround");
        let (nav_page, pref_page, _header, init, _actions) = Self::build_pref_page(&title);

        return Self {
            nav_page,
            pref_page,
            init,
            title,
        };
    }

    fn is_init(&self) -> bool {
        self.init.get_state()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn init(&mut self, application: Rc<Application>) {
        self.on_onit(application);
        self.init.set_state(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.nav_page
    }
}
impl PrefPage for SurroundPage {
    const ACTION_LABEL: &str = "surround";
    const INPUT_ACTION_LABEL: &str = "input";
    const INPUT_PAGE_ACTION_LABEL: &str = "surround.input";
}
impl SurroundPage {
    fn on_onit(&self, application: Rc<Application>) {
        let pipewire = application.pipewire.clone();

        let enable_pref_group = self.build_page_switch();
        self.pref_page.add(&enable_pref_group);

        let section_groups = self.build_sections_from_default(&pipewire.surround);
        for group in section_groups {
            self.pref_page.add(&group);
        }
    }
}

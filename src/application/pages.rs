mod main_page;
mod surround_page;

use super::{
    Application,
    shared::init::{Init, InitTrait},
};
use libadwaita::{
    HeaderBar, NavigationPage, NavigationSplitView, PreferencesPage, ToolbarView,
    gtk::{self, Orientation, prelude::WidgetExt},
};
use main_page::MainPage;
use std::rc::Rc;
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

    fn build_pref_page(title: &str) -> (NavigationPage, PreferencesPage, HeaderBar, Init) {
        let pref_page = PreferencesPage::new();
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&pref_page));

        let nav_page = NavigationPage::builder()
            .title(title)
            .tag(title)
            .child(&toolbar)
            .build();

        let init = Init::new();

        return (nav_page, pref_page, header, init);
    }
}

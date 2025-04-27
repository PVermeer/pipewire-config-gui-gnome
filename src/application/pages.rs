mod main_page;

use super::{
    Application,
    shared::init::{Init, InitTrait},
};
use libadwaita::{
    HeaderBar, NavigationPage, NavigationSplitView, ToolbarView,
    glib::object::IsA,
    gtk::{self, prelude::WidgetExt},
};
use main_page::MainPage;
use std::rc::Rc;

#[repr(i32)]
pub enum Page {
    Main,
}

pub struct Pages {
    pub main: MainPage,
}
impl Pages {
    pub fn new() -> Self {
        Self {
            main: MainPage::new(),
        }
    }
}

pub trait NavPage {
    const LABEL: &str;

    fn new() -> Self;

    fn init(&mut self, _application: Rc<Application>);

    fn get_navpage(&self) -> &NavigationPage;

    fn load_page(&mut self, application: Rc<Application>, view: &NavigationSplitView) {
        self.init(application);

        let nav_page = self.get_navpage();
        if nav_page.parent().is_some() {
            return;
        };
        view.set_content(Some(nav_page));
    }

    fn build_nav_page(
        title: &str,
        content: &impl IsA<gtk::Widget>,
    ) -> (NavigationPage, HeaderBar, Init) {
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(content));

        let page = NavigationPage::builder()
            .title(title)
            .tag(title)
            .child(&toolbar)
            .build();

        let init = Init::new();

        return (page, header, init);
    }
}

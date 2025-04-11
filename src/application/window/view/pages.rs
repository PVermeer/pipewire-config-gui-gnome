mod main_page;
pub mod sidebar_page;

use std::collections::HashMap;

use libadwaita::{
    HeaderBar, NavigationPage, ToolbarView,
    glib::object::IsA,
    gtk::{self},
};
use main_page::MainPage;

pub type Pages = HashMap<Page, PageVariant>;

#[repr(i32)]
#[derive(Eq, Hash, PartialEq)]
pub enum Page {
    Main,
}

pub enum PageVariant {
    Main(MainPage),
}
impl PageVariant {
    pub fn build_hash_map() -> Pages {
        HashMap::from([(Page::Main, PageVariant::Main(MainPage::new()))])
    }

    pub fn get_nav_page(&self) -> &NavigationPage {
        match self {
            Self::Main(value) => &value.page,
        }
    }
}

pub trait NavPage {
    const LABEL: &str;

    fn new() -> Self;

    fn build_nav_page(title: &str, content: &impl IsA<gtk::Widget>) -> (NavigationPage, HeaderBar) {
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(content));

        let page = NavigationPage::builder()
            .title(title)
            .tag(title)
            .child(&toolbar)
            .build();

        return (page, header);
    }
}

pub mod main_page;
mod sidebar_page;

use std::collections::HashMap;

use libadwaita::{
    HeaderBar, NavigationPage, ToolbarView,
    glib::object::IsA,
    gtk::{self, ListBox},
};
use main_page::MainPage;
use sidebar_page::SidebarPage;

pub type Pages = HashMap<Page, PageVariant>;

#[repr(i32)]
#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Page {
    Sidebar,
    Main,
}

#[derive(Clone)]
pub enum PageVariant {
    Sidebar(SidebarPage),
    Main(MainPage),
}
impl PageVariant {
    pub fn build_hash_map() -> Pages {
        HashMap::from([
            (Page::Sidebar, PageVariant::Sidebar(SidebarPage::new())),
            (Page::Main, PageVariant::Main(MainPage::new())),
        ])
    }

    pub fn get_nav_page(&self) -> &NavigationPage {
        match self {
            Self::Sidebar(value) => &value.page,
            Self::Main(value) => &value.page,
        }
    }

    pub fn get_header(&self) -> &HeaderBar {
        match self {
            Self::Sidebar(value) => &value.header,
            Self::Main(value) => &value.header,
        }
    }

    /** Only for sidebar */
    pub fn get_list(&self) -> Option<&ListBox> {
        let Self::Sidebar(value) = self else {
            return None;
        };
        Some(&value.list)
    }
}

pub trait NavPage: Clone {
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

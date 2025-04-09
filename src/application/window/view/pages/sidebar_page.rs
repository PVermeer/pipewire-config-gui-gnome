use libadwaita::{
    HeaderBar, NavigationPage, ToolbarView,
    gtk::{ListBox, SelectionMode},
};

use super::NavPage;

#[derive(Clone)]
pub struct SidebarPage {
    pub page: NavigationPage,
    pub header: HeaderBar,
    pub list: ListBox,
}
impl NavPage for SidebarPage {
    const LABEL: &str = "sidebar-page";

    fn new() -> Self {
        let list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .css_classes(["navigation-sidebar"])
            .build();
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&list));

        let page = NavigationPage::builder()
            .title("List")
            .tag("sidebar")
            .child(&toolbar)
            .build();

        return Self { page, header, list };
    }
}


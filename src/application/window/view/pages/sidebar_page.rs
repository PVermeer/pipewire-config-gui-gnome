use libadwaita::{
    ActionRow, HeaderBar, NavigationPage, ToolbarView,
    glib::Variant,
    gtk::{ListBox, SelectionMode},
};

use crate::application::window::view::View;

use super::{NavPage, Page};

pub struct SidebarPage {
    pub page: NavigationPage,
    pub header: HeaderBar,
    list: ListBox,
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
impl SidebarPage {
    pub fn add_nav_row(&self, title: &str, page: Page) -> ActionRow {
        let action_target = Variant::from(page as i32);

        let row = ActionRow::builder()
            .activatable(true)
            .action_name(View::VIEW_NAVIGATE_ACTION_LABEL)
            .action_target(&action_target)
            .title(title)
            .build();

        self.list.append(&row);

        return row;
    }
}

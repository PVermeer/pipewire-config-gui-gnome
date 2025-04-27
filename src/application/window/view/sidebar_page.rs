use super::{NavPage, Page};
use crate::application::{
    shared::init::{Init, InitTrait},
    window::view::View,
};
use libadwaita::{
    ActionRow, HeaderBar, NavigationPage, ToolbarView,
    glib::Variant,
    gtk::{ListBox, SelectionMode},
};

pub struct SidebarPage {
    pub page: NavigationPage,
    pub header: HeaderBar,
    init: Init,
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

        let init = Init::new();

        return Self {
            page,
            header,
            list,
            init,
        };
    }

    fn init(&mut self, _application: std::rc::Rc<crate::application::Application>) {
        self.init.set_state(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.page
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

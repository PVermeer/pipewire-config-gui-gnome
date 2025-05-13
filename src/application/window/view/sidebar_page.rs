use super::{NavPage, Page};
use crate::application::{pages::PageState, window::view::View};
use libadwaita::{
    ActionRow, HeaderBar, NavigationPage, ToolbarView,
    glib::Variant,
    gtk::{ListBox, SelectionMode},
};

pub struct SidebarPage {
    pub nav_page: NavigationPage,
    pub header: HeaderBar,
    state: PageState,
    title: String,
    list: ListBox,
}
impl NavPage for SidebarPage {
    const LABEL: &str = "sidebar-page";
    const LOG_TARGET: &str = Self::LABEL;

    fn new() -> Self {
        let title = String::from("List");
        let list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .css_classes(["navigation-sidebar"])
            .build();
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&list));

        let nav_page = NavigationPage::builder()
            .title(&title)
            .tag("sidebar")
            .child(&toolbar)
            .build();

        let state = PageState::new();

        return Self {
            nav_page,
            header,
            list,
            state,
            title,
        };
    }

    fn is_init(&self) -> bool {
        self.get_state().get_init()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_state(&self) -> &PageState {
        &self.state
    }

    fn init(&mut self, _application: std::rc::Rc<crate::application::Application>) {
        self.state.set_init(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.nav_page
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

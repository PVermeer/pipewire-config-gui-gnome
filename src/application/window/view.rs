mod main_page;
mod sidebar_page;

use std::collections::HashMap;

use libadwaita::{
    ActionRow, Breakpoint, BreakpointCondition, HeaderBar, NavigationPage, NavigationSplitView,
    ToolbarView,
    gio::{ActionEntry, SimpleActionGroup, prelude::ActionMapExtManual},
    glib::{Value, Variant, VariantTy, object::IsA},
    gtk::{self, ListBox},
};
use main_page::MainPage;
use sidebar_page::SidebarPage;

type Pages = HashMap<String, (NavigationPage, HeaderBar)>;

pub struct View {
    pub pages: Pages,
    pub split_view: NavigationSplitView,
    pub sidebar: SidebarPage,
    pub breakpoint: Breakpoint,
    pub actions: SimpleActionGroup,
}
impl View {
    pub const VIEW_ACTION_LABEL: &str = "view";

    pub fn new() -> Self {
        let mut pages: Pages = HashMap::new();
        let actions = SimpleActionGroup::new();
        let sidebar = SidebarPage::new();
        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar.page)
            .show_content(true)
            .build();
        let breakpoint = Self::build_breakpoint();
        breakpoint.add_setter(&split_view, "collapsed", Some(&Value::from(true)));

        let main_page = MainPage::new();

        pages.insert(
            MainPage::LABEL.to_owned(),
            (main_page.page, main_page.header),
        );

        let build_action = Self::build_navigate_action(&split_view, &pages);
        actions.add_action_entries([build_action]);

        Self::add_nav_row(&sidebar.list, "Main page", "main-page");

        return Self {
            pages,
            split_view,
            sidebar,
            breakpoint,
            actions,
        };
    }

    pub fn navigate(&self, page_label: &str) {
        let (page, _) = self.pages.get(page_label).unwrap();
        self.split_view.set_content(Some(page));
    }

    fn build_navigate_action(
        split_view: &NavigationSplitView,
        pages: &Pages,
    ) -> ActionEntry<SimpleActionGroup> {
        let split_view_clone = split_view.clone();
        let pages_clone = pages.clone();
        let action = ActionEntry::builder("navigate")
            .parameter_type(Some(VariantTy::STRING))
            .activate(move |_: &SimpleActionGroup, _, parameter| {
                let page_label = parameter.unwrap().try_get::<String>().unwrap();
                let (page, _) = pages_clone.get(&page_label).unwrap();
                split_view_clone.set_content(Some(page));
            })
            .build();

        return action;
    }

    fn build_breakpoint() -> Breakpoint {
        let breakpoint_condition = BreakpointCondition::new_length(
            libadwaita::BreakpointConditionLengthType::MaxWidth,
            600_f64,
            libadwaita::LengthUnit::Sp,
        );
        let breakpoint = Breakpoint::new(breakpoint_condition);

        return breakpoint;
    }

    fn add_nav_row(sidebar_list: &ListBox, title: &str, action_name: &str) -> ActionRow {
        let action_target = Variant::from(action_name);

        let row = ActionRow::builder()
            .activatable(true)
            .action_name(View::VIEW_ACTION_LABEL.to_owned() + "." + "navigate")
            .action_target(&action_target)
            .title(title)
            .build();

        sidebar_list.append(&row);
        return row;
    }
}

trait NavPage {
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

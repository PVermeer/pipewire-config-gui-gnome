pub mod pages;

use libadwaita::{
    ActionRow, Breakpoint, BreakpointCondition, NavigationSplitView,
    gio::{ActionEntry, SimpleActionGroup, prelude::ActionMapExtManual},
    glib::{Value, Variant, VariantTy},
    gtk::ListBox,
};
use pages::{Page, PageVariant, Pages};

pub struct View {
    pub pages: Pages,
    pub split_view: NavigationSplitView,
    pub breakpoint: Breakpoint,
    pub actions: SimpleActionGroup,
}
impl View {
    pub const VIEW_ACTION_LABEL: &str = "view";

    pub fn new() -> Self {
        let pages = PageVariant::build_hash_map();
        let sidebar = pages.get(&Page::Sidebar).unwrap();
        let sidebar_page = sidebar.get_nav_page();
        let sidebar_nav_list = sidebar.get_list().unwrap();
        let split_view = NavigationSplitView::builder()
            .sidebar(sidebar_page)
            .show_content(true)
            .build();
        let actions = SimpleActionGroup::new();
        let breakpoint = Self::build_breakpoint();
        let build_action = Self::build_navigate_action(&split_view, &pages);

        breakpoint.add_setter(&split_view, "collapsed", Some(&Value::from(true)));
        actions.add_action_entries([build_action]);
        Self::add_nav_row(sidebar_nav_list, "Main page", Page::Main);

        return Self {
            pages,
            split_view,
            breakpoint,
            actions,
        };
    }

    pub fn navigate(&self, page: Page) {
        let page = self.pages.get(&page).unwrap().get_nav_page();
        self.split_view.set_content(Some(page));
    }

    fn build_navigate_action(
        split_view: &NavigationSplitView,
        pages: &Pages,
    ) -> ActionEntry<SimpleActionGroup> {
        let split_view_clone = split_view.clone();
        let pages_clone = pages.clone();

        let action = ActionEntry::builder("navigate")
            .parameter_type(Some(VariantTy::INT32))
            .activate(move |_, _, parameter| {
                // Using an int as parameter to map back to the enum
                let enum_page_index = parameter.unwrap().try_get::<i32>().unwrap();
                let page_enum: Page = unsafe { ::std::mem::transmute(enum_page_index) };

                let page = pages_clone.get(&page_enum).unwrap().get_nav_page();
                // FIXME - parent error on settings this
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

    fn add_nav_row(sidebar_list: &ListBox, title: &str, page: Page) -> ActionRow {
        let action_target = Variant::from(page as i32);

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

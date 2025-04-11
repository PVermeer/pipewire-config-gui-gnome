pub mod pages;

use libadwaita::{
    Breakpoint, BreakpointCondition, NavigationSplitView,
    gio::{
        ActionEntry, SimpleActionGroup,
        prelude::{ActionGroupExt, ActionMapExtManual},
    },
    glib::{Value, Variant, VariantTy},
    gtk::prelude::WidgetExt,
};
use pages::{NavPage, Page, PageVariant, sidebar_page::SidebarPage};

pub struct View {
    pub sidebar: SidebarPage,
    pub split_view: NavigationSplitView,
    pub breakpoint: Breakpoint,
    pub actions: SimpleActionGroup,
}
impl View {
    pub const ACTION_LABEL: &str = "view";
    const NAVIGATE_ACTION_LABEL: &str = "navigate";
    const VIEW_NAVIGATE_ACTION_LABEL: &str = "view.navigate";

    pub fn new() -> Self {
        let sidebar = SidebarPage::new();
        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar.page)
            .show_content(true)
            .build();
        let actions = SimpleActionGroup::new();
        let breakpoint = Self::build_breakpoint();
        let navigation_action = Self::build_navigate_action(&split_view);

        breakpoint.add_setter(&split_view, "collapsed", Some(&Value::from(true)));
        actions.add_action_entries([navigation_action]);
        sidebar.add_nav_row("Main page", Page::Main);

        return Self {
            sidebar,
            split_view,
            breakpoint,
            actions,
        };
    }

    pub fn navigate(&self, page: Page) {
        let action_target = Variant::from(page as i32);
        self.actions
            .activate_action(Self::NAVIGATE_ACTION_LABEL, Some(&action_target));
    }

    fn build_navigate_action(split_view: &NavigationSplitView) -> ActionEntry<SimpleActionGroup> {
        let split_view_ref = split_view.clone();
        let pages = PageVariant::build_hash_map();

        let action = ActionEntry::builder(View::NAVIGATE_ACTION_LABEL)
            .parameter_type(Some(VariantTy::INT32))
            .activate(move |_, _, parameter| {
                // Using an int as parameter to map back to the enum
                let enum_page_index = parameter.unwrap().try_get::<i32>().unwrap();
                let page_enum: Page = unsafe { ::std::mem::transmute(enum_page_index) };

                let page = pages.get(&page_enum).unwrap().get_nav_page();
                if page.parent().is_some() {
                    return;
                };
                split_view_ref.set_content(Some(page));
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
}

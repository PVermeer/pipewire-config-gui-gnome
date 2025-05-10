pub mod app_menu;
mod sidebar_page;

use crate::application::{
    Application,
    pages::{NavPage, Page},
};
use app_menu::AppMenu;
use libadwaita::{
    Breakpoint, BreakpointCondition, NavigationSplitView,
    gio::{
        ActionEntry, SimpleActionGroup,
        prelude::{ActionGroupExt, ActionMapExtManual},
    },
    glib::{Value, Variant, VariantTy},
};
use sidebar_page::SidebarPage;
use std::rc::Rc;

pub struct View {
    pub app_menu: AppMenu,
    pub sidebar: SidebarPage,
    pub split_view: NavigationSplitView,
    pub breakpoint: Breakpoint,
    pub actions: SimpleActionGroup,
}
impl View {
    pub const ACTION_LABEL: &str = "view";
    pub const NAVIGATE_ACTION_LABEL: &str = "navigate";
    pub const VIEW_NAVIGATE_ACTION_LABEL: &str = "view.navigate";

    pub fn new() -> Self {
        let sidebar = SidebarPage::new();
        let app_menu = AppMenu::new();
        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar.nav_page)
            .show_content(true)
            .build();
        let actions = SimpleActionGroup::new();
        let breakpoint = Self::build_breakpoint();

        Self {
            app_menu,
            sidebar,
            split_view,
            breakpoint,
            actions,
        }
    }

    pub fn init(&self, application: &Rc<Application>) {
        let navigation_action = self.build_navigate_action(&self.split_view, application);

        self.sidebar.header.pack_end(&self.app_menu.button);
        self.breakpoint
            .add_setter(&self.split_view, "collapsed", Some(&Value::from(true)));
        self.actions.add_action_entries([navigation_action]);
    }

    pub fn navigate(&self, page: Page) {
        let action_target = Variant::from(page as i32);
        self.actions
            .activate_action(Self::NAVIGATE_ACTION_LABEL, Some(&action_target));
    }

    fn build_navigate_action(
        &self,
        split_view: &NavigationSplitView,
        application: &Rc<Application>,
    ) -> ActionEntry<SimpleActionGroup> {
        let split_view_ref = split_view.clone();
        let application = application.clone();
        let pages = application.pages.clone();

        let action = ActionEntry::builder(View::NAVIGATE_ACTION_LABEL)
            .parameter_type(Some(VariantTy::INT32))
            .activate(move |_, _, parameter| {
                // Using an int as parameter to map back to the enum
                let enum_page_index = parameter.unwrap().try_get::<i32>().unwrap();
                let page_enum: Page = unsafe { ::std::mem::transmute(enum_page_index) };
                let mut pages_mut = pages.borrow_mut();

                match page_enum {
                    Page::Main => pages_mut
                        .main
                        .load_page(application.clone(), &split_view_ref),
                    Page::Surround => pages_mut
                        .surround
                        .load_page(application.clone(), &split_view_ref),
                };
            })
            .build();

        action
    }

    fn build_breakpoint() -> Breakpoint {
        let breakpoint_condition = BreakpointCondition::new_length(
            libadwaita::BreakpointConditionLengthType::MaxWidth,
            600_f64,
            libadwaita::LengthUnit::Sp,
        );
        let breakpoint = Breakpoint::new(breakpoint_condition);

        breakpoint
    }
}

use libadwaita::{
    AboutWindow, HeaderBar, NavigationPage, NavigationSplitView, ToolbarView,
    gio::{ActionEntry, Menu, MenuItem, SimpleActionGroup, prelude::ActionMapExtManual},
    glib::object::IsA,
    gtk::{
        self, ListBox, MenuButton, Orientation, SelectionMode,
        prelude::{GtkWindowExt, WidgetExt},
    },
};

use crate::config;

pub struct MainMenu {
    menu: Menu,
    button: MenuButton,
    actions: SimpleActionGroup,
}

pub struct ApplicationWindow {
    pub window: libadwaita::ApplicationWindow,
    pub main_menu: MainMenu,
    pub sidebar_list: ListBox,
    pub content_box: gtk::Box,
}
impl ApplicationWindow {
    const MAIN_MENU_ACTION_LABEL: &str = "main-menu";

    pub fn new(adw_application: &libadwaita::Application, title: &str) -> Self {
        let title = title.to_string();
        let list = Self::build_list();
        let content_box = gtk::Box::new(Orientation::Vertical, 0);

        let main_menu_actions = SimpleActionGroup::new();
        let (main_menu_button, main_menu) = Self::build_menu(&main_menu_actions);
        let main_menu = MainMenu {
            menu: main_menu,
            button: main_menu_button,
            actions: main_menu_actions,
        };

        let (sidebar, sidebar_header) = Self::build_view(&title, &list);
        let (content, _content_header) = Self::build_view("Content", &content_box);
        sidebar_header.pack_end(&main_menu.button);

        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar)
            .content(&content)
            .build();

        let window = libadwaita::ApplicationWindow::builder()
            .application(adw_application)
            .title(&title)
            .default_height(600)
            .default_width(800)
            .content(&split_view)
            .build();
        window.insert_action_group(Self::MAIN_MENU_ACTION_LABEL, Some(&main_menu.actions));

        let application_window = Self {
            window,
            main_menu,
            sidebar_list: list,
            content_box,
        };

        return application_window;
    }

    pub fn show(&self) {
        self.window.present();
    }

    fn show_about() {
        let about = AboutWindow::new();
        about.set_application_name(config::APP_NAME);
        about.set_version(config::VERSION);
        about.set_developer_name("Me");
        about.add_credit_section(Some("Code by"), &["Me"]);
        about.add_acknowledgement_section(None, &["Also me"]);
        about.add_legal_section("Some title", None, gtk::License::Gpl30, None);
        about.show();
    }

    fn build_view(title: &str, content: &impl IsA<gtk::Widget>) -> (NavigationPage, HeaderBar) {
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(content));

        let page = NavigationPage::builder()
            .title(title)
            .child(&toolbar)
            .build();

        return (page, header);
    }

    fn build_list() -> ListBox {
        return ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
    }

    fn build_menu(action_group: &SimpleActionGroup) -> (MenuButton, Menu) {
        // GTK does not let a popovermenu to be created programmatically
        // https://blog.libove.org/posts/rust-gtk--creating-a-menu-bar-programmatically-with-gtk-rs
        let menu_button = MenuButton::builder()
            .name("main-menu")
            .icon_name("open-menu-symbolic")
            .build();
        let main_menu = Menu::new();

        main_menu.prepend_item(&Self::build_menu_item(
            "About",
            ("about", || Self::show_about()),
            action_group,
        ));

        menu_button.set_menu_model(Some(&main_menu));
        return (menu_button, main_menu);
    }

    fn build_menu_item(
        label: &str,
        (action_name, action): (&str, impl Fn() + 'static),
        action_group: &SimpleActionGroup,
    ) -> MenuItem {
        let menu_item = MenuItem::new(
            Some(label),
            Some(&(Self::MAIN_MENU_ACTION_LABEL.to_owned() + "." + action_name)),
        );
        let menu_item_action = ActionEntry::builder(action_name)
            .activate(move |_: &SimpleActionGroup, _, _| {
                action();
            })
            .build();
        action_group.add_action_entries([menu_item_action]);

        return menu_item;
    }
}

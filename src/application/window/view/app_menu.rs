use libadwaita::{
    gio::{ActionEntry, Menu, MenuItem, SimpleActionGroup, prelude::ActionMapExtManual},
    gtk::MenuButton,
};

use crate::application::window::ApplicationWindow;

pub struct AppMenu {
    pub button: MenuButton,
    pub actions: SimpleActionGroup,
}
impl AppMenu {
    pub const NAME: &str = "app-menu";
    pub const ACTION_LABEL: &str = "app-menu";

    pub fn new() -> Self {
        // GTK does not let a popovermenu to be created programmatically
        // https://blog.libove.org/posts/rust-gtk--creating-a-menu-bar-programmatically-with-gtk-rs
        let button = MenuButton::builder()
            .name(AppMenu::NAME)
            .icon_name("open-menu-symbolic")
            .build();
        let menu = Menu::new();
        let actions = SimpleActionGroup::new();

        button.set_menu_model(Some(&menu));

        Self::add_about(&menu, &actions);

        return Self { button, actions };
    }

    fn add_about(menu: &Menu, actions: &SimpleActionGroup) {
        let item =
            Self::build_menu_item("About", ("about", ApplicationWindow::show_about), actions);
        menu.prepend_item(&item);
    }

    fn build_menu_item(
        label: &str,
        (action_name, action): (&str, impl Fn() + 'static),
        actions: &SimpleActionGroup,
    ) -> MenuItem {
        let item = MenuItem::new(
            Some(label),
            Some(&(Self::ACTION_LABEL.to_owned() + "." + action_name)),
        );
        let action = ActionEntry::builder(action_name)
            .activate(move |_: &SimpleActionGroup, _, _| {
                action();
            })
            .build();
        actions.add_action_entries([action]);

        return item;
    }
}

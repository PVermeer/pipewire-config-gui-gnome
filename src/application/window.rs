mod app_menu;
mod view;

use crate::config;
use app_menu::AppMenu;
use libadwaita::{
    AboutWindow,
    gtk::{
        self,
        prelude::{GtkWindowExt, WidgetExt},
    },
    prelude::AdwApplicationWindowExt,
};
use view::{View, pages::Page};

pub struct ApplicationWindow {
    pub window: libadwaita::ApplicationWindow,
}
impl ApplicationWindow {
    pub fn new(adw_application: &libadwaita::Application, title: &str) -> Self {
        let title = title.to_string();
        let app_menu = AppMenu::new();
        let view = View::new();
        let window = libadwaita::ApplicationWindow::builder()
            .application(adw_application)
            .title(&title)
            .default_height(600)
            .default_width(800)
            .content(&view.split_view)
            .build();

        window.insert_action_group(AppMenu::ACTION_LABEL, Some(&app_menu.actions));
        window.insert_action_group(View::ACTION_LABEL, Some(&view.actions));
        window.add_breakpoint(view.breakpoint.clone());

        view.sidebar.header.pack_end(&app_menu.button);
        view.navigate(Page::Main);

        return Self { window };
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
}

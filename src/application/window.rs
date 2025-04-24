pub mod view;

use super::Application;
use crate::config;
use libadwaita::{
    AboutWindow,
    gtk::{
        self,
        prelude::{GtkWindowExt, WidgetExt},
    },
    prelude::AdwApplicationWindowExt,
};
use std::rc::Rc;
use view::{View, app_menu::AppMenu};

pub struct ApplicationWindow {
    pub window: libadwaita::ApplicationWindow,
    pub view: View,
}
impl ApplicationWindow {
    pub fn new(adw_application: &libadwaita::Application) -> Self {
        let title = config::APP_NAME.to_string();
        let view = View::new();
        let window = libadwaita::ApplicationWindow::builder()
            .application(adw_application)
            .title(&title)
            .default_height(600)
            .default_width(800)
            .content(&view.split_view)
            .build();

        return Self { window, view };
    }

    pub fn init(&self, application: &Rc<Application>) {
        self.view.init(application);
        self.window
            .insert_action_group(AppMenu::ACTION_LABEL, Some(&self.view.app_menu.actions));
        self.window
            .insert_action_group(View::ACTION_LABEL, Some(&self.view.actions));
        self.window.add_breakpoint(self.view.breakpoint.clone());

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

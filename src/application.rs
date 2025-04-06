mod window;

use libadwaita::{ActionRow, prelude::ActionRowExt};

use crate::config;

pub struct Application {
    pub adw_application: libadwaita::Application,
    pub window: window::ApplicationWindow,
}

impl Application {
    pub fn new(adw_application: &libadwaita::Application) -> Self {
        let window = window::ApplicationWindow::new(&adw_application, config::APP_NAME);

        return Self {
            adw_application: adw_application.clone(),
            window,
        };
    }

    pub fn run_app(&self) {
        self.window.show();
        self.add_nav_row("Some row", |_| println!("Clicked row!"));
    }

    fn add_nav_row(&self, title: &str, on_activate: fn(action_row: &ActionRow)) -> ActionRow {
        let row = ActionRow::builder().activatable(true).title(title).build();
        row.connect_activated(on_activate);
        self.window.sidebar_list.append(&row);

        return row;
    }
}

mod application;
mod config;

use application::Application;
use libadwaita::gio::prelude::{ApplicationExt, ApplicationExtManual};

fn main() {
    let adw_application = libadwaita::Application::builder()
        .application_id(config::APP_ID)
        .build();

    adw_application.connect_activate(|adw_application| {
        let application = Application::new(adw_application);
        application.run_app();
    });

    adw_application.run();
}

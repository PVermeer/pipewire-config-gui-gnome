mod application;
mod config;

use application::Application;
use libadwaita::gio::prelude::{ApplicationExt, ApplicationExtManual};

#[tokio::main]
async fn main() {
    let adw_application: libadwaita::Application = libadwaita::Application::builder()
        .application_id(config::APP_ID)
        .build();

    adw_application.connect_activate(|adw_application| {
        let application = Application::new(adw_application.clone());
        application.run_app();
    });

    adw_application.run();
}

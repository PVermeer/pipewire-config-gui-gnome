mod application;
mod config;

use application::Application;
use libadwaita::gio::prelude::{ApplicationExt, ApplicationExtManual};

fn build_application(adw_application: &libadwaita::Application) {
    let application = Application {
        app_id: config::APP_ID.to_string(),
        version: config::VERSION.to_string(),
        application: adw_application,
        window: application::window::ApplicationWindow::new(adw_application, config::APP_ID),
    };

    application.window.show();
    application
        .window
        .connect_to_row_click(|_| println!("Clicked"));
}

#[tokio::main]
async fn main() {
    let application = libadwaita::Application::new(Some(config::APP_ID), Default::default());
    application.connect_activate(build_application);
    application.run();
}

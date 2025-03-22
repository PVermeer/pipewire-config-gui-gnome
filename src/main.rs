mod config;
mod window;

use config::APP_ID;
use libadwaita::{
    Application,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use window::ApplicationWindow;

fn on_activate_connect(application: &Application) {
    let application_window = ApplicationWindow::new(application, APP_ID);
    application_window.connect_to_row_click(&application_window.row, || {
        eprintln!("Clicked again!");
    });
    application_window.show();
}

#[tokio::main]
async fn main() {
    let application = Application::new(Some(APP_ID), Default::default());

    application.connect_activate(|application| {
        on_activate_connect(application);
    });

    application.run();
}

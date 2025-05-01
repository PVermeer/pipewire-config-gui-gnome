mod application;
mod config;

use anyhow::Result;
use application::Application;
use env_logger::Env;
use libadwaita::gio::prelude::{ApplicationExt, ApplicationExtManual};

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let adw_application = libadwaita::Application::builder()
        .application_id(config::APP_ID)
        .build();

    adw_application.connect_activate(|adw_application| {
        let application = Application::new(adw_application);
        Application::run_app(&application);
    });

    adw_application.run();

    Ok(())
}

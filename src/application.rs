mod window;

use window::ApplicationWindow;

use crate::config;

pub struct Application {
    pub adw_application: libadwaita::Application,
    pub window: ApplicationWindow,
}

impl Application {
    pub fn new(adw_application: &libadwaita::Application) -> Self {
        let window = ApplicationWindow::new(adw_application, config::APP_NAME);

        return Self {
            adw_application: adw_application.to_owned(),
            window,
        };
    }

    pub fn run_app(&self) {
        self.window.show();
    }
}

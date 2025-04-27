mod pages;
mod pipewire;
mod shared;
mod window;

use pages::{Page, Pages};
use pipewire::Pipewire;
use std::{cell::RefCell, rc::Rc};
use window::ApplicationWindow;

pub struct Application {
    pub window: ApplicationWindow,
    pub pipewire: Rc<Pipewire>,
    pub pages: Rc<RefCell<Pages>>,
}
impl Application {
    pub fn new(adw_application: &libadwaita::Application) -> Rc<Self> {
        let pipewire = Pipewire::new();
        let window = ApplicationWindow::new(adw_application);
        let pages = Rc::new(RefCell::new(Pages::new()));

        return Rc::new(Self {
            window,
            pipewire,
            pages,
        });
    }

    pub fn run_app(application: &Rc<Application>) {
        application.window.init(application);
        application.navigate(Page::Main);
    }

    fn navigate(&self, page: Page) {
        self.window.view.navigate(page);
    }
}

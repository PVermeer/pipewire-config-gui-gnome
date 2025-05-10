mod pages;
mod pipewire;
mod shared;
mod window;

use pages::{NavPage, Page, Pages};
use pipewire::pipewire::Pipewire;
use std::{cell::RefCell, rc::Rc};
use window::ApplicationWindow;

pub struct Application {
    pub window: ApplicationWindow,
    pub pipewire: Rc<Pipewire>,
    pub pages: Rc<RefCell<Pages>>,
}
impl Application {
    pub fn new(adw_application: &libadwaita::Application) -> Rc<Self> {
        let pipewire = match Pipewire::new() {
            Ok(pipewire) => pipewire,
            Err(error) => todo!("Pipewire class error handling:\n{:?}", error),
        };
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

        let sidebar = &application.window.view.sidebar;
        {
            // Scoped to drop borrow before continuing
            let pages = application.pages.borrow();
            sidebar.add_nav_row(pages.main.get_title(), Page::Main);
            sidebar.add_nav_row(pages.surround.get_title(), Page::Surround);
        }

        application.navigate(Page::Surround);
    }

    fn navigate(&self, page: Page) {
        self.window.view.navigate(page);
    }
}

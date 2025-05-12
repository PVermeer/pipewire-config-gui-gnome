use super::NavPage;
use crate::application::{
    Application,
    pipewire::pipewire::Pipewire,
    shared::init::{Init, InitTrait},
};
use libadwaita::{
    NavigationPage,
    glib::{self},
    gtk::{
        self, Button, Label,
        prelude::{BoxExt, ButtonExt},
    },
};
use log::info;
use std::rc::Rc;

pub struct MainPage {
    pub nav_page: NavigationPage,
    button: Button,
    init: Init,
    title: String,
}
impl NavPage for MainPage {
    const LABEL: &str = "main-page";
    const LOG_TARGET: &str = Self::LABEL;

    fn new() -> Self {
        let title = String::from("Main page");
        let label = Label::builder()
            .label(concat!(
                "<b>Label title</b>\n",
                "<span>Label text</span>\n",
                "<span>Some more text</span>"
            ))
            .wrap(true)
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();

        let button = Button::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .label("Open file")
            .build();

        let (nav_page, _header, content_box, init) = Self::build_nav_page(&title);

        content_box.append(&label);
        content_box.append(&button);

        return Self {
            nav_page,
            button,
            init,
            title,
        };
    }

    fn init(&mut self, application: Rc<Application>) {
        self.on_init(application);
        self.init.set_state(true);
    }

    fn is_init(&self) -> bool {
        self.init.get_state()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.nav_page
    }
}
impl MainPage {
    fn on_init(&mut self, application: Rc<Application>) {
        let pipewire = application.pipewire.clone();

        self.button.connect_clicked(move |_| {
            let pipewire = pipewire.clone();
            glib::spawn_future_local(async { Self::get_document(pipewire).await });
        });
    }

    async fn get_document(pipewire: Rc<Pipewire>) {
        let a = &pipewire.surround.borrow().current;
        info!("{:?}", a);
        todo!()
    }
}

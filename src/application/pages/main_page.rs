use super::NavPage;
use crate::application::{
    Application,
    pipewire::Pipewire,
    shared::init::{Init, InitTrait},
};
use libadwaita::{
    NavigationPage,
    glib::{self},
    gtk::{
        self, Button, Label, Orientation,
        prelude::{BoxExt, ButtonExt},
    },
};
use log::info;
use std::rc::Rc;

pub struct MainPage {
    pub page: NavigationPage,
    pub button: Button,
    init: Init,
}
impl NavPage for MainPage {
    const LABEL: &str = "main-page";

    fn new() -> Self {
        let content_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

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

        content_box.append(&label);
        content_box.append(&button);

        let (page, _header, init) = Self::build_nav_page("Content", &content_box);

        return Self { page, button, init };
    }

    fn init(&mut self, application: Rc<Application>) {
        if self.init.get_state() {
            return;
        }

        let pipewire = application.pipewire.clone();

        self.button.connect_clicked(move |_| {
            let pipewire = pipewire.clone();
            glib::spawn_future_local(async { Self::get_document(pipewire).await });
        });

        self.init.set_state(true);
    }

    fn get_navpage(&self) -> &NavigationPage {
        &self.page
    }
}
impl MainPage {
    async fn get_document(pipewire: Rc<Pipewire>) {
        let a = &pipewire.default_config;
        info!("{}", a);
        todo!()
    }
}

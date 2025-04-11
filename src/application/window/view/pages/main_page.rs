use libadwaita::{
    NavigationPage,
    gtk::{self, Label, Orientation, prelude::BoxExt},
};

use super::NavPage;

pub struct MainPage {
    pub page: NavigationPage,
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

        content_box.append(&label);
        let (page, _header) = Self::build_nav_page("Content", &content_box);

        return Self { page };
    }
}

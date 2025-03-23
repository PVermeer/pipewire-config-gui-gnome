use libadwaita::{
    ActionRow, HeaderBar,
    gtk::{Box, ListBox, Orientation, SelectionMode, prelude::GtkWindowExt},
    prelude::{ActionRowExt, BoxExt},
};

#[allow(dead_code)]
pub struct ApplicationWindow {
    pub window: libadwaita::ApplicationWindow,
    pub title: String,
    pub header: HeaderBar,
    pub row: ActionRow,
    pub list: ListBox,
}
impl ApplicationWindow {
    fn build_header() -> HeaderBar {
        let header = HeaderBar::new();
        return header;
    }

    fn build_list() -> ListBox {
        return ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
    }

    fn build_row() -> ActionRow {
        return ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
    }

    pub fn connect_to_row_click(&self, callback: fn(&ActionRow)) {
        self.row.connect_activated(callback);
    }

    pub fn show(&self) {
        self.window.present();
    }

    pub fn new(application: &libadwaita::Application, title: &str) -> Self {
        let title = title.to_string();
        let header = Self::build_header();
        let list = Self::build_list();
        let row = Self::build_row();

        list.append(&row);

        let content = Box::new(Orientation::Vertical, 0);
        content.append(&header);
        content.append(&list);

        let window = libadwaita::ApplicationWindow::builder()
            .application(application)
            .title(&title)
            .default_height(600)
            .default_width(800)
            .content(&content)
            .build();

        let application_window = Self {
            title,
            header,
            list,
            row,
            window,
        };

        return application_window;
    }
}

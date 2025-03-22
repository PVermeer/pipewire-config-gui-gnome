use libadwaita::{
    ActionRow, Application, ApplicationWindow, HeaderBar,
    gtk::{Box, ListBox, Orientation, SelectionMode},
    prelude::{ActionRowExt, ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
};

const APP_ID: &str = "org.pvermeer.home-finance-gnome";

fn build_ui(application: &Application) {
    let header = HeaderBar::new();

    let row = ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        // makes the list look nicer
        .css_classes(vec![String::from("boxed-list")])
        .build();
    list.append(&row);

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&header);
    content.append(&list);

    let window = ApplicationWindow::builder()
        .application(application)
        .title(APP_ID)
        .default_height(600)
        .default_width(800)
        .content(&content)
        .build();
    window.show();
}

pub fn main() {
    let application = Application::new(Some(APP_ID), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

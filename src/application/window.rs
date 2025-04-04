use libadwaita::{
    HeaderBar, NavigationPage, NavigationSplitView, ToolbarView,
    glib::object::IsA,
    gtk::{self, ListBox, Orientation, SelectionMode, prelude::GtkWindowExt},
};

pub struct ApplicationWindow {
    pub window: libadwaita::ApplicationWindow,
    pub sidebar_list: ListBox,
    pub content_box: gtk::Box,
}
impl ApplicationWindow {
    pub fn new(adw_application: &libadwaita::Application, title: &str) -> Self {
        let title = title.to_string();
        let list = Self::build_list();
        let content_box = gtk::Box::new(Orientation::Vertical, 0);

        let sidebar = Self::build_view(&title, &list);
        let content = Self::build_view("Content", &content_box);

        let split_view = NavigationSplitView::builder()
            .sidebar(&sidebar)
            .content(&content)
            .build();

        let window = libadwaita::ApplicationWindow::builder()
            .application(adw_application)
            .title(&title)
            .default_height(600)
            .default_width(800)
            .content(&split_view)
            .build();

        let application_window = Self {
            window,
            sidebar_list: list,
            content_box,
        };

        return application_window;
    }

    pub fn show(&self) {
        self.window.present();
    }

    fn build_view(title: &str, content: &impl IsA<gtk::Widget>) -> NavigationPage {
        let header = HeaderBar::new();
        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(content));

        let page = NavigationPage::builder()
            .title(title)
            .child(&toolbar)
            .build();

        return page;
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
}

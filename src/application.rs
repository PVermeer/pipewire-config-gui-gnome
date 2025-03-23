pub mod window;

#[allow(dead_code)]
pub struct Application<'a> {
    pub application: &'a libadwaita::Application,
    pub window: window::ApplicationWindow,
    pub app_id: String,
    pub version: String,
}

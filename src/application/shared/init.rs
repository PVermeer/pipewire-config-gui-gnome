pub trait InitTrait {
    fn new() -> Self;
    fn get_state(&self) -> bool;
    fn set_state(&mut self, state: bool);
}
pub struct Init {
    init: bool,
}
impl InitTrait for Init {
    fn new() -> Self {
        Self { init: false }
    }

    fn get_state(&self) -> bool {
        return self.init;
    }

    fn set_state(&mut self, state: bool) {
        self.init = state;
    }
}

pub struct State {
    pub show_log: bool,
}

impl State {
    pub fn new() -> Self {
        Self { show_log: false }
    }
}

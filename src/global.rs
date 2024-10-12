use serde::Serialize;


#[derive(Default, Serialize)]
pub struct Global {
    pub tick: u64,

}

impl Global {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
use std::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub invisible: bool
}

pub fn get_config() -> Config {
    return *(&*CONFIG).lock().unwrap().as_ref().expect("Cannot get config before init");
}
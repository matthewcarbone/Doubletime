use std::path::PathBuf;
use homedir::GetHomeError;
use homedir::get_my_home;

pub fn get_home() -> Result<Option<PathBuf>, GetHomeError> {
    get_my_home().unwrap().unwrap().as_path()?
}

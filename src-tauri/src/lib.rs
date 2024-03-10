use std::path::PathBuf;

use anyhow::Result;
use dirs::home_dir;
// use life_progress_core;

fn get_config_path() -> Result<PathBuf> {
    if let Some(home) = home_dir() {
        Ok(home.join(".life_progress/config.toml"))
    } else {
        Ok(PathBuf::from(".life_progress/config.toml"))
    }
}

pub fn is_init_done() -> Result<bool> {
    Ok(get_config_path()?.is_file())
}

pub fn generate_progree_icon() -> Result<()> {
    let path = get_config_path()?.to_str().expect("Need config");
    Ok(())
}

pub fn init() -> Result<bool> {
    if let Ok(true) = is_init_done() {
        Ok(true)
    } else {
        Ok(false)
    }
}

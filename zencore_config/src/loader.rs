use std::fs;
use std::io::Write;

use crate::{
    config::Config, 
    error::ErrorConf, 
    path::config_file_path,
};

pub fn autostart_and_load() -> Result<Config, ErrorConf> {
    let path = config_file_path().ok_or(ErrorConf::PathResolution)?;

    if !path.exists() {
        create_default(&path)?;
        return Ok(Config::default());
    }

    let raw = fs::read_to_string(&path)?;
    let conf: Config = toml::from_str(&raw)?;

    Ok(conf)
}

pub fn create_default(path: &camino::Utf8PathBuf) -> Result<(), ErrorConf> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let default = Config::default();
    let content = toml::to_string_pretty(&default).unwrap();

    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

use crate::{config::Config, error::ErrorConf};
use std::fs;

pub fn validate_config(conf: &Config) -> Result<(), ErrorConf> {
    // Check workspace path
    if !conf.paths.workspace.exists() {
        return Err(ErrorConf::InvalidValue(
            "Workspace path doesn't exists!".into(),
        ));
    }

    // Check temporary dir / or cache dir
    if let Some(parent) = conf.paths.temp_dir.parent() {
        if !parent.exists() {
            return Err(ErrorConf::InvalidValue("temp dir doesn't exists".into()));
        }
    }

    // Check theme config
    if !conf.progress.theme_file.exists() {
        return Err(ErrorConf::InvalidValue("Theme file doesn't exists".into()));
    }

    if conf.progress.refresh_rate_ms == 0 {
        return Err(ErrorConf::InvalidValue(
            "Refresh rate must be greater than 0".into(),
        ));
    }

    Ok(())
}

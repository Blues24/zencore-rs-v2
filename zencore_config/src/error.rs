use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorConf {
    #[error("Cannot find config directory")]
    PathResolution,

    #[error("Something wrong with I/O ")]
    Io(#[from] std::io::Error),

    #[error("Cannot parse config file")]
    Parse(#[from] toml::de::Error),

    #[error("Invalid config value at: {0}")]
    InvalidValue(String),
}

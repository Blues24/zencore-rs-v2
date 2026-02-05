use camino::Utf8PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    // IO & filesystem
    #[error("failed to read theme file `{0}`: {1}")]
    Io(Utf8PathBuf, #[source] std::io::Error),

    // Parsing error (TOML → struct)
    #[error("failed to parse theme file `{0}`: {1}")]
    Parse(Utf8PathBuf, String),

    // Semantic error (value tidak dikenal, token invalid)
    #[error("invalid theme value: {0}")]
    InvalidValue(String),

    // Logical error (resolver / invariant)
    #[error("theme resolution error: {0}")]
    Resolve(String),
}

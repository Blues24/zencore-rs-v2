use serde::Deserialize;

#[derive(Debug, Deserialize)
struct RawCommonTheme {
    colors: RawColors,
    symbols: RawSymbols,
}

#[derive(Debug, Deserialize)]
struct RawColors {
    primary: String,
    secondary: String,
    success: String,
    warning: String,
    error: String,
    muted: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawSymbols {
    info: char,
    success: char,
    warning: char,
    error: char,
    arrow: char,
}

use crate::common::ColorToken;
use crate::error::ThemeError;

fn parse_color(colorVal: &str) -> Result<ColorToken, ThemeError> {
    match colorVal {
        "primary" => Ok(ColorToken::Primary),
        "secondary" => Ok(ColorToken::Secondary),
        "success" => Ok(ColorToken::Success),
        "warning" => Ok(ColorToken::Warning),
        "error" => Ok(ColorToken::Error),
        "muted" => Ok(ColorToken::Muted),
        _ => Err(ThemeError::InvalidValue(format!(
            "unknown color token: {value}"
        ))),
    }
}

use crate::common::{CommonTheme, Colors, Symbols, Emphasis};

impl RawCommonTheme {
    fn into_common_theme(self) -> Result<CommonTheme, ThemeError> {
        Ok(CommonTheme {
            colors: Colors {
                primary: parse_color(&self.colors.primary)?,
                secondary: parse_color(&self.colors.secondary)?,
                success: parse_color(&self.colors.success)?,
                warning: parse_color(&self.colors.warning)?,
                error: parse_color(&self.colors.error)?,
                muted: parse_color(
                    self.colors
                        .muted
                        .as_deref()
                        .unwrap_or("muted"),
                )?,
            },
            symbols: Symbols {
                info: self.symbols.info,
                success: self.symbols.success,
                warning: self.symbols.warning,
                error: self.symbols.error,
                arrow: self.symbols.arrow,
            },
            emphasis: Emphasis::default(),
        })
    }
}

use std::fs;
use camino::Utf8PathBuf;

pub fn load_common_theme(path: &Utf8PathBuf) -> Result<CommonTheme, ThemeError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ThemeError::Io(path.clone(), e))?;

    let raw: RawCommonTheme = toml::from_str(&content)
        .map_err(|e| ThemeError::Parse(path.clone(), e))?;

    raw.into_common_theme()
}

//! Zencore theme engine (common layer)
//!
//! This crate provides renderer-agnostic theme primitives
//! and loading/resolution logic for CLI and TUI frontends.

mod common;
mod error;
mod loader;
mod resolver;

pub use common::{
    ColorToken, Colors, CommonThemes, Emphasis, Symbols, TextIntensity, TextStyles, TextWeight,
};

pub use error::ThemeError;
pub use loader::load_common_theme;
pub use resolver::ThemeResolver;

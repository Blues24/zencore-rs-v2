//! Zencore Archive - Core archive library
//!
//! This library provides the core functionality for creating compressed archives
//! from directories. It supports multiple archive formats (tar.gz, tar.zst, zip)
//! and provides flexible configuration for filtering, sorting, and error handling.
//!
//! # Design Philosophy
//!
//! - **Library-first**: No CLI, no TUI, no terminal output
//! - **Sync-only**: Sequential I/O, no async complexity
//! - **Progress via callbacks**: Caller controls how progress is displayed
//! - **Configurable error handling**: Fail fast, skip invalid, or best effort
//! - **Zero panics**: All errors are returned, never panic
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use zencore_archive::{ArchiveConfig, ArchiveFormat, create_archive, no_progress};
//! use std::path::PathBuf;
//!
//! let config = ArchiveConfig::new(
//!     PathBuf::from("./my-project"),
//!     PathBuf::from("./backup.tar.gz"),
//!     ArchiveFormat::TarGz,
//! );
//!
//! create_archive(&config, no_progress())?;
//! # Ok::<(), zencore_archive::ArchiveError>(())
//! ```
//!
//! # With Progress Callback
//!
//! ```rust,no_run
//! use zencore_archive::{ArchiveConfig, ProgressCallback, ProgressEvent};
//!
//! struct MyProgress;
//!
//! impl ProgressCallback for MyProgress {
//!     fn on_progress(&mut self, event: ProgressEvent) {
//!         match event {
//!             ProgressEvent::FileCompleted { path, index, total, .. } => {
//!                 println!("[{}/{}] {}", index, total, path);
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```

// Public API
pub mod config;
pub mod error;
pub mod progress;

// Internal modules (not part of public API yet - Day 2+)
mod filter;
mod walker;
// mod sorter;
// mod metadata;
// mod formats;

// Re-exports for convenience
pub use config::{
    ArchiveConfig, ArchiveConfigBuilder, ArchiveFormat, CompressionLevel, ErrorStrategy,
    SortStrategy,
};
pub use error::{ArchiveError, ErrorCollector, Result};
pub use progress::{BoxedProgress, NoOpProgress, ProgressCallback, ProgressEvent, no_progress};

/// Create an archive from a directory
///
/// This is the main entry point for creating archives. It will:
/// 1. Scan the source directory and collect files
/// 2. Sort files according to the sort strategy
/// 3. Create the archive with progress reporting
/// 4. Handle errors according to the error strategy
///
/// # Arguments
///
/// * `config` - Archive configuration
/// * `progress` - Progress callback (use `no_progress()` for none)
///
/// # Returns
///
/// * `Ok(())` if archive was created successfully
/// * `Err(ArchiveError)` if a fatal error occurred
/// * `Err(ArchiveError::Multiple)` if using SkipInvalid and errors occurred
///
/// # Example
///
/// ```rust,no_run
/// use zencore_archive::*;
/// use std::path::PathBuf;
///
/// let config = ArchiveConfig::new(
///     PathBuf::from("./src"),
///     PathBuf::from("./backup.tar.gz"),
///     ArchiveFormat::TarGz,
/// );
///
/// create_archive(&config, no_progress())?;
/// # Ok::<(), ArchiveError>(())
/// ```
pub fn create_archive(_config: &ArchiveConfig, mut _progress: BoxedProgress) -> Result<()> {
    // TODO: Implement in Day 2-4
    // This is a placeholder for Day 1
    todo!("Implementation coming in Day 2-4")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_creation() {
        let config = ArchiveConfig::new(
            PathBuf::from("."),
            PathBuf::from("test.tar.gz"),
            ArchiveFormat::TarGz,
        );

        assert_eq!(config.format, ArchiveFormat::TarGz);
        assert_eq!(config.sort_strategy, SortStrategy::LargestFirst);
        assert_eq!(config.error_strategy, ErrorStrategy::SkipInvalid);
    }

    #[test]
    fn test_config_builder() {
        let config = ArchiveConfigBuilder::new(
            PathBuf::from("."),
            PathBuf::from("test.tar.gz"),
            ArchiveFormat::TarGz,
        )
        .compression(CompressionLevel::Best)
        .sort_strategy(SortStrategy::Alphabetical)
        .include_hidden(true)
        .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.sort_strategy, SortStrategy::Alphabetical);
        assert!(config.include_hidden);
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(ArchiveFormat::TarGz.extension(), "tar.gz");
        assert_eq!(ArchiveFormat::TarZst.extension(), "tar.zst");
        assert_eq!(ArchiveFormat::Zip.extension(), "zip");
    }

    #[test]
    fn test_format_from_extension() {
        assert_eq!(
            ArchiveFormat::from_extension("tar.gz"),
            Some(ArchiveFormat::TarGz)
        );
        assert_eq!(
            ArchiveFormat::from_extension("tgz"),
            Some(ArchiveFormat::TarGz)
        );
        assert_eq!(
            ArchiveFormat::from_extension("tar.zst"),
            Some(ArchiveFormat::TarZst)
        );
        assert_eq!(
            ArchiveFormat::from_extension("zip"),
            Some(ArchiveFormat::Zip)
        );
        assert_eq!(ArchiveFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_error_collector() {
        let mut collector = ErrorCollector::new();
        assert!(!collector.has_errors());

        collector.add(ArchiveError::InvalidConfig("test".to_string()));
        assert!(collector.has_errors());
        assert_eq!(collector.count(), 1);

        let result = collector.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn test_no_op_progress() {
        let mut progress = NoOpProgress;
        progress.on_progress(ProgressEvent::ScanStarted {
            path: "test".to_string(),
        });
        assert!(!progress.should_cancel());
    }
}

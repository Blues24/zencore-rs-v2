use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for zencore-archive operations
pub type Result<T> = std::result::Result<T, ArchiveError>;

/// Main error type for archive operations
#[derive(Error, Debug)]
pub enum ArchiveError {
    /// IO error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error from ignore crate
    #[error("Ignore error: {0}")]
    Ignore(#[from] ignore::Error),

    /// Error walking directory tree
    #[error("Failed to walk directory: {0}")]
    WalkDir(String),

    /// Error reading file metadata
    #[error("Failed to read metadata for {path}: {source}")]
    Metadata {
        path: PathBuf,
        source: std::io::Error,
    },

    /// File was corrupted or unreadable
    #[error("Corrupted file: {path}")]
    CorruptedFile { path: PathBuf },

    /// Archive format not supported
    #[error("Unsupported archive format: {0}")]
    UnsupportedFormat(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Path is invalid or doesn't exist
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    /// Archive creation failed
    #[error("Failed to create archive: {0}")]
    ArchiveCreation(String),

    /// Compression error
    #[error("Compression error: {0}")]
    Compression(String),

    /// Multiple errors occurred (collect mode)
    #[error("Multiple errors occurred: {count} errors")]
    Multiple {
        count: usize,
        errors: Vec<ArchiveError>,
    },
}

impl ArchiveError {
    /// Create a metadata error with context
    pub fn metadata(path: PathBuf, source: std::io::Error) -> Self {
        Self::Metadata { path, source }
    }

    /// Create a corrupted file error
    pub fn corrupted(path: PathBuf) -> Self {
        Self::CorruptedFile { path }
    }

    /// Check if error is recoverable (can skip and continue)
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::CorruptedFile { .. } | Self::Metadata { .. })
    }
}

/// Collected errors during archive operations
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<ArchiveError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an error to the collector
    pub fn add(&mut self, error: ArchiveError) {
        self.errors.push(error);
    }

    /// Check if any errors were collected
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get count of collected errors
    pub fn count(&self) -> usize {
        self.errors.len()
    }

    /// Consume collector and return error if any exist
    pub fn into_result(self) -> Result<()> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(ArchiveError::Multiple {
                count: self.errors.len(),
                errors: self.errors,
            })
        }
    }

    /// Get reference to all errors
    pub fn errors(&self) -> &[ArchiveError] {
        &self.errors
    }
}

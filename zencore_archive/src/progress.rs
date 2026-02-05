use std::path::Path;

/// Progress event emitted during archive operations
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// Started scanning directory
    ScanStarted {
        path: String,
    },

    /// File discovered during scan
    FileDiscovered {
        path: String,
        size: u64,
    },

    /// Scan completed
    ScanCompleted {
        total_files: usize,
        total_size: u64,
    },

    /// Started sorting files
    SortingStarted {
        file_count: usize,
    },

    /// Sorting completed
    SortingCompleted,

    /// Archive creation started
    ArchiveStarted {
        format: String,
        output: String,
    },

    /// Processing a file
    FileProcessing {
        path: String,
        size: u64,
        index: usize,
        total: usize,
    },

    /// File successfully added to archive
    FileCompleted {
        path: String,
        compressed_size: u64,
        index: usize,
        total: usize,
    },

    /// File skipped due to error
    FileSkipped {
        path: String,
        reason: String,
    },

    /// Archive creation completed
    ArchiveCompleted {
        output: String,
        total_files: usize,
        total_size: u64,
        compressed_size: u64,
    },

    /// Error occurred (non-fatal if SkipInvalid strategy)
    Error {
        message: String,
        recoverable: bool,
    },
}

impl ProgressEvent {
    /// Create a file processing event
    pub fn processing(path: &Path, size: u64, index: usize, total: usize) -> Self {
        Self::FileProcessing {
            path: path.display().to_string(),
            size,
            index,
            total,
        }
    }

    /// Create a file completed event
    pub fn completed(
        path: &Path,
        compressed_size: u64,
        index: usize,
        total: usize,
    ) -> Self {
        Self::FileCompleted {
            path: path.display().to_string(),
            compressed_size,
            index,
            total,
        }
    }

    /// Create a file skipped event
    pub fn skipped(path: &Path, reason: impl Into<String>) -> Self {
        Self::FileSkipped {
            path: path.display().to_string(),
            reason: reason.into(),
        }
    }
}

/// Trait for progress reporting callbacks
///
/// Implement this trait to receive progress updates during archive operations.
/// The trait is object-safe, allowing for dynamic dispatch.
pub trait ProgressCallback: Send {
    /// Called when a progress event occurs
    fn on_progress(&mut self, event: ProgressEvent);

    /// Check if operation should be cancelled
    ///
    /// Return true to request cancellation. The archive operation will
    /// stop at the next safe point and return an error.
    fn should_cancel(&self) -> bool {
        false
    }
}

/// No-op progress callback (default)
pub struct NoOpProgress;

impl ProgressCallback for NoOpProgress {
    fn on_progress(&mut self, _event: ProgressEvent) {}
}

/// Simple progress callback that prints to stdout (for testing)
#[cfg(test)]
pub struct PrintProgress;

#[cfg(test)]
impl ProgressCallback for PrintProgress {
    fn on_progress(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::FileProcessing { path, index, total, .. } => {
                println!("[{}/{}] Processing: {}", index, total, path);
            }
            ProgressEvent::FileCompleted { path, index, total, .. } => {
                println!("[{}/{}] Completed: {}", index, total, path);
            }
            ProgressEvent::FileSkipped { path, reason } => {
                println!("Skipped: {} ({})", path, reason);
            }
            ProgressEvent::ArchiveCompleted { total_files, compressed_size, .. } => {
                println!("Archive completed: {} files, {} bytes", total_files, compressed_size);
            }
            _ => {}
        }
    }
}

/// Helper to create boxed progress callbacks
pub type BoxedProgress = Box<dyn ProgressCallback>;

/// Create a no-op progress callback
pub fn no_progress() -> BoxedProgress {
    Box::new(NoOpProgress)
}

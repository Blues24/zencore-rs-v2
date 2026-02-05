use crate::config::ArchiveConfig;
use crate::error::{ArchiveError, ErrorCollector, Result};
use crate::filter::FileFilter;
use crate::progress::{BoxedProgress, ProgressEvent};
use std::path::PathBuf;
use walkdir::WalkDir;

/// File entry with metadata
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Absolute path to file
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,

    /// Relative path from archive root
    pub relative_path: PathBuf,
}

impl FileEntry {
    /// Create FileEntry from path
    pub fn new(path: PathBuf, base_path: &std::path::Path) -> Result<Self> {
        let metadata =
            std::fs::metadata(&path).map_err(|e| ArchiveError::metadata(path.clone(), e))?;

        let relative_path = path
            .strip_prefix(base_path)
            .map_err(|_| ArchiveError::InvalidPath(path.clone()))?
            .to_path_buf();

        Ok(Self {
            path,
            size: metadata.len(),
            relative_path,
        })
    }
}

/// Walk directory and collect files
pub fn walk_directory(
    config: &ArchiveConfig,
    progress: &mut BoxedProgress,
) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    let mut errors = ErrorCollector::new();

    // Create filter
    let filter = FileFilter::new(
        &config.source,
        config.respect_gitignore,
        config.include_hidden,
        &config.exclude_patterns,
        &config.include_patterns,
    )?;

    // Emit scan started event
    progress.on_progress(ProgressEvent::ScanStarted {
        path: config.source.display().to_string(),
    });

    // Walk directory tree
    let walker = WalkDir::new(&config.source)
        .follow_links(config.follow_symlinks)
        .into_iter();

    for entry in walker {
        // Check cancellation
        if progress.should_cancel() {
            return Err(ArchiveError::ArchiveCreation(
                "Operation cancelled by user".to_string(),
            ));
        }

        // Handle walk errors
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                let err = ArchiveError::WalkDir(e.to_string());
                if config.error_strategy == crate::config::ErrorStrategy::FailFast {
                    return Err(err);
                }
                errors.add(err);
                continue;
            }
        };

        // Skip directories (we only archive files)
        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();

        // Apply filter
        if !filter.should_include(path) {
            continue;
        }

        // Create file entry
        match FileEntry::new(path.to_path_buf(), &config.source) {
            Ok(file_entry) => {
                // Emit file discovered event
                progress.on_progress(ProgressEvent::FileDiscovered {
                    path: file_entry.relative_path.display().to_string(),
                    size: file_entry.size,
                });

                entries.push(file_entry);
            }
            Err(e) => {
                if config.error_strategy == crate::config::ErrorStrategy::FailFast {
                    return Err(e);
                }
                errors.add(e);
            }
        }
    }

    // Calculate total size
    let total_size: u64 = entries.iter().map(|e| e.size).sum();

    // Emit scan completed event
    progress.on_progress(ProgressEvent::ScanCompleted {
        total_files: entries.len(),
        total_size,
    });

    // Return errors if any (SkipInvalid mode)
    if errors.has_errors() && config.error_strategy != crate::config::ErrorStrategy::BestEffort {
        return errors.into_result().map(|_| entries);
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ArchiveConfig, ArchiveFormat};
    use crate::progress::no_progress;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_tree() -> TempDir {
        let temp = TempDir::new().unwrap();
        let base = temp.path();

        // Create test files
        fs::write(base.join("file1.txt"), "content1").unwrap();
        fs::write(base.join("file2.txt"), "content2").unwrap();

        // Create subdirectory
        fs::create_dir(base.join("subdir")).unwrap();
        fs::write(base.join("subdir/file3.txt"), "content3").unwrap();

        // Create hidden file
        fs::write(base.join(".hidden"), "hidden").unwrap();

        temp
    }

    #[test]
    fn test_walk_basic() {
        let temp = create_test_tree();
        let config = ArchiveConfig::new(
            temp.path().to_path_buf(),
            PathBuf::from("test.tar.gz"),
            ArchiveFormat::TarGz,
        );

        let mut progress = no_progress();
        let entries = walk_directory(&config, &mut progress).unwrap();

        // Should find 3 visible files (file1, file2, file3)
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_walk_include_hidden() {
        let temp = create_test_tree();
        let mut config = ArchiveConfig::new(
            temp.path().to_path_buf(),
            PathBuf::from("test.tar.gz"),
            ArchiveFormat::TarGz,
        );
        config.include_hidden = true;

        let mut progress = no_progress();
        let entries = walk_directory(&config, &mut progress).unwrap();

        // Should find 4 files (including .hidden)
        assert_eq!(entries.len(), 4);
    }

    #[test]
    fn test_relative_paths() {
        let temp = create_test_tree();
        let config = ArchiveConfig::new(
            temp.path().to_path_buf(),
            PathBuf::from("test.tar.gz"),
            ArchiveFormat::TarGz,
        );

        let mut progress = no_progress();
        let entries = walk_directory(&config, &mut progress).unwrap();

        // Check relative paths are correct
        let has_subdir_file = entries
            .iter()
            .any(|e| e.relative_path == PathBuf::from("subdir/file3.txt"));

        assert!(has_subdir_file);
    }
}


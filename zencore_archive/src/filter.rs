use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// File filter that combines gitignore and glob patterns
pub struct FileFilter {
    /// Gitignore matcher (if enabled)
    gitignore: Option<Gitignore>,

    /// Glob patterns to exclude
    exclude_globs: Vec<glob::Pattern>,

    /// Glob patterns to include (empty = include all)
    include_globs: Vec<glob::Pattern>,

    /// Include hidden files
    include_hidden: bool,
}

impl FileFilter {
    /// Create new file filter
    pub fn new(
        base_path: &Path,
        respect_gitignore: bool,
        include_hidden: bool,
        exclude_patterns: &[String],
        include_patterns: &[String],
    ) -> crate::Result<Self> {
        // Build gitignore matcher
        let gitignore = if respect_gitignore {
            let mut builder = GitignoreBuilder::new(base_path);

            // Add .gitignore files from base path upwards
            let gitignore_path = base_path.join(".gitignore");
            if gitignore_path.exists() {
                builder.add(gitignore_path);
            }

            Some(builder.build()?)
        } else {
            None
        };

        // Compile glob patterns
        let exclude_globs = exclude_patterns
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();

        let include_globs = include_patterns
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();

        Ok(Self {
            gitignore,
            exclude_globs,
            include_globs,
            include_hidden,
        })
    }

    /// Check if a path should be included
    pub fn should_include(&self, path: &Path) -> bool {
        // Check hidden files
        if !self.include_hidden && is_hidden(path) {
            return false;
        }

        // Check gitignore
        if let Some(ref gitignore) = self.gitignore {
            let matched = gitignore.matched(path, path.is_dir());
            if matched.is_ignore() {
                return false;
            }
        }

        // Check exclude patterns
        if self.matches_any_glob(path, &self.exclude_globs) {
            return false;
        }

        // Check include patterns (if any specified)
        if !self.include_globs.is_empty() {
            return self.matches_any_glob(path, &self.include_globs);
        }

        true
    }

    /// Check if path matches any glob pattern
    fn matches_any_glob(&self, path: &Path, patterns: &[glob::Pattern]) -> bool {
        let path_str = path.to_string_lossy();
        patterns.iter().any(|p| p.matches(&path_str))
    }
}

/// Check if a path is hidden (starts with .)
fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_hidden_detection() {
        assert!(is_hidden(Path::new(".hidden")));
        assert!(is_hidden(Path::new("dir/.hidden")));
        assert!(!is_hidden(Path::new("visible")));
        assert!(!is_hidden(Path::new("dir/visible")));
    }

    #[test]
    fn test_filter_hidden_files() {
        let filter = FileFilter::new(
            Path::new("."),
            false,
            false, // Don't include hidden
            &[],
            &[],
        )
        .unwrap();

        assert!(!filter.should_include(Path::new(".hidden")));
        assert!(filter.should_include(Path::new("visible")));
    }

    #[test]
    fn test_exclude_patterns() {
        let filter = FileFilter::new(
            Path::new("."),
            false,
            true,
            &["*.log".to_string(), "tmp/*".to_string()],
            &[],
        )
        .unwrap();

        assert!(!filter.should_include(Path::new("error.log")));
        assert!(!filter.should_include(Path::new("tmp/cache.dat")));
        assert!(filter.should_include(Path::new("data.txt")));
    }

    #[test]
    fn test_include_patterns() {
        let filter = FileFilter::new(
            Path::new("."),
            false,
            true,
            &[],
            &["*.rs".to_string(), "*.toml".to_string()],
        )
        .unwrap();

        assert!(filter.should_include(Path::new("main.rs")));
        assert!(filter.should_include(Path::new("Cargo.toml")));
        assert!(!filter.should_include(Path::new("readme.md")));
    }
}

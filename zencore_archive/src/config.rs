use std::path::PathBuf

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    TarGz, // .tar.gz file 
    TarZst,// .tar.zst file 
    Zip,// .zip file 
} // Archive format collection, i will be adding more format in the future 

impl ArchiveFormat {
    // Get the file extension for every single chosen format 
    pub fn extension(&self) -> &'static str {
        match self {
            Self::TarGz => "tar.gz",
            Self::TarZst => "tar.zst",
            Self::Zip => "zip",
            // Add more format in this line 
            _ => None 
        }
    }
    
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "tar.gz" | "tgz" => Some(Self::TarGz),
            "tar.zst" | "tzst" => Some(Self::TarZst),
            "zip" | ".zip" => Some(Self::Zip),
            // Add more format in this line 
            _ => None,
        } 
    }
}

/// Strategy for handling errors during archive creation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorStrategy {
    /// Abort on first error
    FailFast,
    /// Skip invalid files, collect errors
    SkipInvalid,
    /// Create partial archive with whatever succeeded
    BestEffort,
}

/// Strategy for sorting files before archiving
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortStrategy {
    /// Largest files first (default)
    LargestFirst,
    /// Smallest files first
    SmallestFirst,
    /// Alphabetical order
    Alphabetical,
    /// Newest modification time first
    MtimeNewest,
    /// No sorting (fastest, streaming-friendly)
    None,
}

/// Compression level for archive formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (store only)
    None,
    /// Fastest compression
    Fast,
    /// Balanced compression (default)
    Default,
    /// Best compression (slowest)
    Best,
    /// Custom level (0-9 for gzip, 1-22 for zstd)
    Custom(u32),
}

impl CompressionLevel {
    /// Convert to flate2 compression level
    pub fn to_flate2(&self) -> flate2::Compression {
        match self {
            Self::None => flate2::Compression::none(),
            Self::Fast => flate2::Compression::fast(),
            Self::Default => flate2::Compression::default(),
            Self::Best => flate2::Compression::best(),
            Self::Custom(level) => flate2::Compression::new(*level.min(&9)),
        }
    }

    /// Convert to zstd compression level
    pub fn to_zstd(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Fast => 1,
            Self::Default => 3,
            Self::Best => 22,
            Self::Custom(level) => (*level as i32).min(22),
        }
    }
}

/// Main configuration for archive operations
#[derive(Debug, Clone)]
pub struct ArchiveConfig {
    /// Source directory to archive
    pub source: PathBuf,

    /// Output archive path
    pub output: PathBuf,

    /// Archive format
    pub format: ArchiveFormat,

    /// Compression level
    pub compression: CompressionLevel,

    /// Error handling strategy
    pub error_strategy: ErrorStrategy,

    /// File sorting strategy
    pub sort_strategy: SortStrategy,

    /// Follow symlinks
    pub follow_symlinks: bool,

    /// Include hidden files (dotfiles)
    pub include_hidden: bool,

    /// Respect .gitignore files
    pub respect_gitignore: bool,

    /// Glob patterns to exclude
    pub exclude_patterns: Vec<String>,

    /// Glob patterns to include (empty = all)
    pub include_patterns: Vec<String>,

    /// Preserve empty directories
    pub preserve_empty_dirs: bool,
}

impl ArchiveConfig {
    /// Create new config with required fields
    pub fn new(source: PathBuf, output: PathBuf, format: ArchiveFormat) -> Self {
        Self {
            source,
            output,
            format,
            compression: CompressionLevel::Default,
            error_strategy: ErrorStrategy::SkipInvalid,
            sort_strategy: SortStrategy::LargestFirst,
            follow_symlinks: false,
            include_hidden: false,
            respect_gitignore: true,
            exclude_patterns: Vec::new(),
            include_patterns: Vec::new(),
            preserve_empty_dirs: true,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        use crate::error::ArchiveError;

        // Check source exists
        if !self.source.exists() {
            return Err(ArchiveError::InvalidPath(self.source.clone()));
        }

        // Check source is directory
        if !self.source.is_dir() {
            return Err(ArchiveError::InvalidConfig(
                "Source must be a directory".to_string(),
            ));
        }

        // Check output parent exists
        if let Some(parent) = self.output.parent() {
            if !parent.exists() {
                return Err(ArchiveError::InvalidPath(parent.to_path_buf()));
            }
        }

        Ok(())
    }
}

/// Builder for ArchiveConfig
pub struct ArchiveConfigBuilder {
    config: ArchiveConfig,
}

impl ArchiveConfigBuilder {
    pub fn new(source: PathBuf, output: PathBuf, format: ArchiveFormat) -> Self {
        Self {
            config: ArchiveConfig::new(source, output, format),
        }
    }

    pub fn compression(mut self, level: CompressionLevel) -> Self {
        self.config.compression = level;
        self
    }

    pub fn error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.config.error_strategy = strategy;
        self
    }

    pub fn sort_strategy(mut self, strategy: SortStrategy) -> Self {
        self.config.sort_strategy = strategy;
        self
    }

    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.config.follow_symlinks = follow;
        self
    }

    pub fn include_hidden(mut self, include: bool) -> Self {
        self.config.include_hidden = include;
        self
    }

    pub fn respect_gitignore(mut self, respect: bool) -> Self {
        self.config.respect_gitignore = respect;
        self
    }

    pub fn exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.exclude_patterns = patterns;
        self
    }

    pub fn include_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.include_patterns = patterns;
        self
    }

    pub fn preserve_empty_dirs(mut self, preserve: bool) -> Self {
        self.config.preserve_empty_dirs = preserve;
        self
    }

    pub fn build(self) -> crate::error::Result<ArchiveConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}


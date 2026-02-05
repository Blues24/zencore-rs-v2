use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use crate::override::OverrideConf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub paths: PathConfig,
    pub progress: ProgressConfig,
}

// General config that handle config file creation
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub first_run: bool,
}

// Path handling config
#[derive(Debug, Serialize, Deserialize)]
pub struct PathConfig {
    pub workspace: Utf8PathBuf,
    pub temp_dir: Utf8PathBuf,
}

// Progress bar and theming config
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressConfig {
    pub theme_file: Utf8PathBuf,
    // Refresh rate in milisecond
    pub refresh_rate_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig { first_run: true },

            paths: PathConfig {
                workspace: Utf8PathBuf::from("./"),
                temp_dir: Utf8PathBuf::from("./zencore.tmp"),
            },

            progress: ProgressConfig {
                theme_file: Utf8PathBuf::from("themes/theme.conf"),
                refresh_rate_ms: 100,
            },
        }
    }
}

impl Config {
    pub fn apply_override(mut self, ovr: OverrideConf) -> Self {
        if let Some(ws) = ovr.workspace{
            self.paths.workspace = ws;
        }

        if let Some(theme) = ovr.theme_file {
            self.progress.theme_file = theme;
        }

        if let Some(ms) = ovr.refresh_rate_ms {
            self.progress.refresh_rate_ms = ms;
        }

        self 
    }
}

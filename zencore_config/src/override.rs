use camino::Utf8PathBuf;

#[derive(Default)]
pub struct OverrideConf {
    pub workspace: Option<Utf8PathBuf>,
    pub theme_file: Option<Utf8PathBuf>,
    pub refresh_rate_ms: Option<Utf8PathBuf>,
}

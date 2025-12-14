use camino::Utf8PathBuf;
use directories::ProjectDirs;

pub fn config_file_path() -> Option<Utf8PathBuf> {
    let projdir = ProjectDirs::from("rs", "blues", "zencore")?;

    let dir = Utf8PathBuf::from_path_buf(projdir.config_dir().to_path_buf()).ok()?;
    Some(dir.join("config.toml"))
}

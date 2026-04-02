use directories::ProjectDirs;
use std::path::PathBuf;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "simworld").unwrap()
}

pub fn data_dir_path() -> PathBuf {
    project_dirs().data_dir().to_path_buf()
}

pub fn app_state_file_path() -> PathBuf {
    data_dir_path().join("app.ron")
}

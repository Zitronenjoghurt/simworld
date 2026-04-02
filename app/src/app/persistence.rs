use crate::app::directories::app_state_file_path;
use crate::app::persistence::window::WindowState;
use crate::ui::AppUi;

pub mod window;

#[derive(serde::Serialize)]
pub struct AppStateWrite<'a> {
    pub app: &'a AppUi,
    pub egui: &'a egui::Memory,
    pub window: WindowState,
}

impl<'a> AppStateWrite<'a> {
    pub fn save(&self) -> anyhow::Result<()> {
        let path = app_state_file_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        ron::Options::default().to_io_writer_pretty(writer, self, Default::default())?;
        Ok(())
    }
}

#[derive(Default, serde::Deserialize)]
pub struct AppStateRead {
    pub app: AppUi,
    pub egui: egui::Memory,
    pub window: WindowState,
}

impl AppStateRead {
    pub fn exists() -> bool {
        app_state_file_path().exists()
    }

    pub fn load() -> anyhow::Result<Option<Self>> {
        let path = app_state_file_path();
        if !path.exists() {
            return Ok(None);
        }

        let file = std::fs::File::open(app_state_file_path())?;
        let reader = std::io::BufReader::new(file);
        Ok(Some(ron::de::from_reader(reader)?))
    }
}

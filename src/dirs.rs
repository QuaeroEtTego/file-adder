use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum Dirs {
    Audio,
    Cache,
    Config,
    ConfigLocal,
    Data,
    DataLocal,
    Desktop,
    Document,
    Download,
    Executable,
    Font,
    Home,
    Picture,
    Preference,
    Public,
    Runtime,
    State,
    Template,
    Video,
}

impl Dirs {
    pub fn execute(&self) -> Option<PathBuf> {
        use self::Dirs::*;

        match self {
            Audio => dirs::audio_dir(),
            Cache => dirs::cache_dir(),
            Config => dirs::config_dir(),
            ConfigLocal => dirs::config_local_dir(),
            Data => dirs::data_dir(),
            DataLocal => dirs::data_local_dir(),
            Desktop => dirs::desktop_dir(),
            Document => dirs::document_dir(),
            Download => dirs::download_dir(),
            Executable => dirs::executable_dir(),
            Font => dirs::font_dir(),
            Home => dirs::home_dir(),
            Picture => dirs::picture_dir(),
            Preference => dirs::preference_dir(),
            Public => dirs::public_dir(),
            Runtime => dirs::runtime_dir(),
            State => dirs::state_dir(),
            Template => dirs::template_dir(),
            Video => dirs::video_dir(),
        }
    }
}

impl FromStr for Dirs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AUDIO_DIR" => Self::Audio,
            "CACHE_DIR" => Self::Cache,
            "CONFIG_DIR" => Self::Config,
            "CONFIG_LOCAL_DIR" => Self::ConfigLocal,
            "DATA_DIR" => Self::Data,
            "DATA_LOCAL_DIR" => Self::DataLocal,
            "DESKTOP_DIR" => Self::Desktop,
            "DOCUMENT_DIR" => Self::Document,
            "DOWNLOAD_DIR" => Self::Download,
            "EXECUTABLE_DIR" => Self::Executable,
            "FONT_DIR" => Self::Font,
            "HOME_DIR" => Self::Home,
            "PICTURE_DIR" => Self::Picture,
            "PREFERENCE_DIR" => Self::Preference,
            "PUBLIC_DIR" => Self::Public,
            "RUNTIME_DIR" => Self::Runtime,
            "STATE_DIR" => Self::State,
            "TEMPLATE_DIR" => Self::Template,
            "VIDEO_DIR" => Self::Video,
            _ => {
                return Err(String::from("Cannot find a dir function"));
            }
        })
    }
}

pub fn parse_path(path: &str) -> PathBuf {
    #[cfg(target_family = "unix")]
    const DELIMITER: char = '/';

    #[cfg(target_family = "windows")]
    const DELIMITER: char = '\\';

    if let Some((left, right)) = path.split_once(DELIMITER) {
        if right.is_empty() {
            return default_path(path);
        }

        if let Ok(dirs) = Dirs::from_str(left) {
            return dirs
                .execute()
                .map_or_else(|| default_path(path), |path_buf| path_buf.join(right));
        }
    }

    default_path(path)
}

fn default_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

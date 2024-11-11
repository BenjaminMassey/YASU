#[derive(serde::Deserialize)]
pub struct Replay {
    pub enabled: bool,
    pub obs_path: String,
    pub video_ext: String,
    pub save_delay: u64,
}

#[derive(serde::Deserialize)]
pub struct Graphics {
    pub enabled: bool,
    pub player_image_size: Vec<u32>,
    pub player_image_fill: bool,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub replay: Replay,
    pub graphics: Graphics,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            replay: Replay {
                enabled: false,
                obs_path: String::new(),
                video_ext: String::new(),
                save_delay: 0u64,
            },
            graphics: Graphics {
                enabled: false,
                player_image_size: vec![],
                player_image_fill: false,
            },
        }
    }
}

pub fn get_settings() -> Settings {
    toml::from_str(&std::fs::read_to_string("./settings.toml").unwrap()).unwrap()
}
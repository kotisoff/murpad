use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::{fs::File, io::BufReader};

use crate::audio_utils;

pub fn load<T: DeserializeOwned + Serialize + Default>(file: &str) -> Result<T, Box<dyn Error>> {
    let f = File::open(file);
    match f {
        Ok(buffer) => {
            let reader = BufReader::new(buffer);
            let data: T = serde_json::from_reader(reader)?;

            Ok(data)
        }
        Err(_) => {
            let data = T::default();
            let _ = save::<T>(&data, file);

            Ok(data)
        }
    }
}

pub fn save<T>(config: &T, file: &str) -> Result<usize, std::io::Error>
where
    T: Serialize,
{
    let mut writer = File::create(file)?;
    let serialized = serde_json::to_string(config)?;
    writer.write(serialized.as_bytes())
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct SoundField {
    pub label: String,
    pub file: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct SoundsFile {
    sounds: Vec<SoundField>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SoundConfig {
    pub device: String,
    pub volume: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SocketConfig {
    pub enabled: bool,
    pub port: i16,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppConfig {
    pub sound: SoundConfig,
    pub socket: SocketConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let (output_devices, default_device) = audio_utils::get_audio_devices().unwrap();

        let device = default_device.or(output_devices.get(0).cloned()).unwrap();
        let volume: f32 = 1.0;

        let sound = SoundConfig { device, volume };

        let enabled = true;
        let port: i16 = 12345;

        let socket = SocketConfig { enabled, port };

        Self { sound, socket }
    }
}

pub fn load_sounds() -> Vec<SoundField> {
    let conf = load::<SoundsFile>("./sounds.json").expect("Конфиг звуков не смог загрузиться.");

    conf.sounds
}

pub fn save_sounds(sounds: &Vec<SoundField>) {
    let conf = SoundsFile {
        sounds: sounds.clone(),
    };

    save::<SoundsFile>(&conf, "./sounds.json").expect("Не удалось сохранить конфиг звуков.");
}

pub fn load_app_config() -> AppConfig {
    load::<AppConfig>("./config.json").expect("Конфиг программы не смог загрузиться.")
}

pub fn save_app_config(config: &AppConfig) {
    save::<AppConfig>(config, "./config.json").expect("Не удалось сохранить конфиг программы");
}

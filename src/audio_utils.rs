use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Context, Result};
use rodio::cpal::default_host;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{Decoder, OutputStreamBuilder, Sink};

use crate::app_config;

pub fn get_audio_devices() -> Result<(Vec<String>, Option<String>)> {
    let host = default_host();
    let devices = host.output_devices()?;

    let mut device_names = Vec::new();
    let mut default_device_name = None;

    // Получаем устройство по умолчанию
    if let Some(default) = host.default_output_device() {
        if let Ok(name) = default.name() {
            default_device_name = Some(name.clone());
            device_names.push(name);
        }
    }

    // Добавляем остальные
    for device in devices {
        if let Ok(name) = device.name() {
            if !device_names.contains(&name) {
                device_names.push(name);
            }
        }
    }

    Ok((device_names, default_device_name))
}

// Воспроизведение звука на конкретном устройстве
pub async fn play_sound_on_device(
    path: PathBuf,
    device_name: String,
    config: app_config::AppConfig,
) {
    let host = default_host();
    let mut devices = host.output_devices().unwrap();

    // Ищем устройство по имени
    let device = devices
        .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
        .with_context(|| format!("Устройство '{}' не найдено", device_name))
        .unwrap();

    // Создаем поток для устройства
    let stream_handler = OutputStreamBuilder::from_device(device)
        .unwrap()
        .open_stream()
        .with_context(|| {
            format!(
                "Не удалось создать источник звука из устройства '{}'.",
                device_name
            )
        })
        .unwrap();

    // Открываем файл
    let file = File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let volume = config.sound.volume;

    let sink = Sink::connect_new(&stream_handler.mixer());
    sink.set_volume(volume);
    sink.append(source);
    sink.sleep_until_end();
}

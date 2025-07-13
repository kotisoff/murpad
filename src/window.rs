use crate::{
    app_config,
    audio_utils::{get_audio_devices, play_sound_on_device},
    sound_button::SoundButton,
    utils::Message,
};

use iced::{
    Alignment, Element, Length,
    widget::{Column, button, column, container, pick_list, text},
};
use std::path::PathBuf;

#[derive(Clone)]
struct SoundPad {
    buttons: Vec<SoundButton>,
    sounds_dir: PathBuf,
    output_devices: Vec<String>,
    selected_device: Option<String>,
    config: config::Config,
}

impl Default for SoundPad {
    fn default() -> Self {
        let sounds_dir = PathBuf::from("sounds");
        let config: config::Config = app_config::load_config();

        let (output_devices, selected_device) = match get_audio_devices() {
            Ok((devices, default)) => (devices, default),
            Err(_) => (Vec::new(), None),
        };

        let mut buttons: Vec<SoundButton> = Vec::new();
        let mut next_id: usize = 0;

        for sound in app_config::load_sounds() {
            let button = SoundButton::new(next_id.clone(), &sound.label, &sound.file);
            next_id = next_id + 1;

            buttons.push(button);
        }

        Self {
            buttons,
            sounds_dir,
            output_devices,
            selected_device,
            config,
        }
    }
}

impl SoundPad {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(id) => {
                if let Some(button) = self.buttons.get(id) {
                    let sound_path = self.sounds_dir.join(&button.file);
                    if let Some(device_name) = &self.selected_device {
                        let _ = play_sound_on_device(sound_path, device_name, self.config.clone());
                    }
                }
            }
            Message::OutputDeviceSelected(device_name) => {
                self.selected_device = Some(device_name);
            }
            Message::RefreshDevices => {
                if let Ok((devices, default)) = get_audio_devices() {
                    self.output_devices = devices;
                    self.selected_device = default;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let children: Vec<Element<'_, Message>> = self
            .buttons
            .iter()
            .map(SoundButton::view)
            .map(Element::from)
            .collect();

        let sound_buttons = Column::with_children(children).spacing(20);

        let device_list = pick_list(
            self.output_devices.clone(),
            self.selected_device.clone(),
            Message::OutputDeviceSelected,
        )
        .width(Length::Fixed(300.0));

        let device_picker = column![
            text("Выберите устройство вывода:"),
            device_list,
            button(text("Обновить список устройств")).on_press(Message::RefreshDevices),
        ]
        .spacing(10);

        container(
            column![device_picker, sound_buttons]
                .spacing(30)
                .align_x(Alignment::Center),
        )
        .center(Length::Fill)
        .into()
    }
}

pub fn create_window() -> iced::Result {
    iced::run("MurPad", SoundPad::update, SoundPad::view)
}

use crate::{
    app_config,
    audio_utils::{get_audio_devices, play_sound_on_device},
    socket,
    sound_button::SoundButton,
    utils::Message,
};

use iced::{
    Alignment, Element, Length, Task,
    widget::{Column, button, column, container, pick_list, text},
};
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Clone)]
struct SoundPad {
    buttons: Vec<SoundButton>,
    sounds_dir: PathBuf,
    output_devices: Vec<String>,
    config: app_config::AppConfig,
}

impl Default for SoundPad {
    fn default() -> Self {
        let sounds_dir = PathBuf::from("sounds");
        let config: app_config::AppConfig = app_config::load_app_config();

        let (output_devices, _selected_device) = match get_audio_devices() {
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
            config,
        }
    }
}

impl SoundPad {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ButtonPressed(id) => {
                if let Some(button) = self.buttons.get(id) {
                    let sound_path = self.sounds_dir.join(&button.file);
                    let device_name = &self.config.sound.device;
                    let config = self.config.clone();

                    return Task::<Message>::perform(
                        play_sound_on_device(sound_path, device_name.to_string(), config),
                        |_| Message::None,
                    );
                }
            }
            Message::OutputDeviceSelected(device_name) => {
                self.config.sound.device = device_name;
                app_config::save_app_config(&self.config);
            }
            Message::RefreshDevices => {
                if let Ok((devices, default)) = get_audio_devices() {
                    self.output_devices = devices;
                    let device_name = self.config.sound.device.clone();
                    self.config.sound.device = default.unwrap_or(device_name);
                }
            }
            Message::TokioStartListening => {
                let (tx, mut rx) = mpsc::channel::<usize>(10);

                tokio::task::spawn(async move {
                    let config = app_config::load_app_config();
                    println!("{:?}", config);

                    if config.socket.enabled {
                        let server = socket::Server::new(config.socket.port).await;
                        println!("Слушоем пакеты");
                        server.listen_packets(tx).await;
                    };
                });

                let obj = self.clone();

                return Task::<Message>::perform(
                    async move {
                        while let Some(msg) = rx.recv().await {
                            println!("Жесть, пакет, {}", msg);

                            if let Some(button) = obj.buttons.get(msg - 1) {
                                let sound_path = obj.sounds_dir.join(&button.file);
                                let device_name = &obj.config.sound.device;
                                let config = obj.config.clone();

                                play_sound_on_device(sound_path, device_name.to_string(), config)
                                    .await;
                            }
                        }
                        Message::TokioMessageReceived(0)
                    },
                    |m| m,
                );
            }
            Message::TokioMessageReceived(id) => return Task::done(Message::ButtonPressed(id)),
            _ => {}
        }
        Task::none()
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
            Some(self.config.sound.device.clone()),
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
    let app = iced::application("MurPad", SoundPad::update, SoundPad::view);
    app.run_with(|| {
        (
            SoundPad::default(),
            Task::done(Message::TokioStartListening),
        )
    })

    // iced::run("MurPad", SoundPad::update, SoundPad::view)
}

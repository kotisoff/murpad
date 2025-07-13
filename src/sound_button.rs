use crate::utils::Message;
use iced::widget::{Button, button, text};

#[derive(Debug, Clone)]
pub struct SoundButton {
    pub id: usize,
    pub label: String,
    pub file: String,
}

impl SoundButton {
    pub fn new(id: usize, label: &str, file: &str) -> Self {
        SoundButton {
            id,
            label: label.to_string(),
            file: file.to_string(),
        }
    }

    pub fn view(&self) -> Button<'_, Message> {
        button(text(&self.label)).on_press(Message::ButtonPressed(*&self.id))
    }
}

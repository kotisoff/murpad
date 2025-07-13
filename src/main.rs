mod app_config;
mod audio_utils;
mod sound_button;
mod utils;
mod window;

fn main() -> iced::Result {
    window::create_window()
}

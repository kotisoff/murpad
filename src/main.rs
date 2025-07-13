mod app_config;
mod audio_utils;
mod socket;
mod sound_button;
mod utils;
mod window;

#[tokio::main]
async fn main() {
    window::create_window().unwrap();
}

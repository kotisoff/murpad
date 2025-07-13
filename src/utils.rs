#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(usize),
    OutputDeviceSelected(String),
    RefreshDevices,
    TokioStartListening,
    TokioMessageReceived(usize),
    None,
}

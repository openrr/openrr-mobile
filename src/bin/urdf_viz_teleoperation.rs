use std::sync::Arc;

use arci_gamepad_gilrs::{GilGamepad, GilGamepadConfig};
use arci_urdf_viz::UrdfVizWebClient;
use openrr_client::PrintSpeaker;
use openrr_mobile::teleop_manager::TeleopManager;

#[tokio::main]
async fn main() {
    let speaker = Arc::new(PrintSpeaker::new());
    let move_base = Arc::new(UrdfVizWebClient::default());
    move_base.run_send_velocity_thread();
    let gamepad = Arc::new(GilGamepad::new_from_config(GilGamepadConfig::default()));

    let teleop_manager = TeleopManager::new(move_base, speaker, gamepad);

    loop {
        teleop_manager.update().await;
        teleop_manager.speak().await;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

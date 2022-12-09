use std::sync::Arc;

use openrr_client::PrintSpeaker;
use openrr_mobile::teleop_manager::TeleopManager;

#[cfg(feature = "ros")]
use arci_ros::{RosCmdVelMoveBase, RosJoyGamepad, RosJoyGamepadConfig};

#[cfg(not(feature = "ros"))]
use arci_gamepad_gilrs::{GilGamepad, GilGamepadConfig};
#[cfg(not(feature = "ros"))]
use arci_urdf_viz::UrdfVizWebClient;

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    let mut move_base: _;
    #[allow(unused_mut)]
    let mut gamepad: _;

    let speaker = Arc::new(PrintSpeaker::new());

    #[cfg(feature = "ros")]
    {
        arci_ros::init("mobile");
        move_base = Arc::new(RosCmdVelMoveBase::new("/cmd_vel"));
        gamepad = Arc::new(RosJoyGamepad::new_from_config(
            &RosJoyGamepadConfig::default(),
        ));
    }

    #[cfg(not(feature = "ros"))]
    {
        move_base = Arc::new(UrdfVizWebClient::default());
        move_base.run_send_velocity_thread();
        gamepad = Arc::new(GilGamepad::new_from_config(GilGamepadConfig::default()));
    }

    let teleop_manager = TeleopManager::new(move_base, speaker, gamepad);

    loop {
        teleop_manager.update().await;
        teleop_manager.speak().await;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

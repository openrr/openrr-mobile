#[cfg(feature = "ros")]
#[tokio::main]
async fn main() {
    use arci_gamepad_gilrs::{GilGamepad, GilGamepadConfig};
    use arci_ros::RosCmdVelMoveBase;
    use openrr_client::PrintSpeaker;
    use openrr_mobile::teleop_manager::TeleopManager;
    use std::sync::Arc;

    // Initialize
    arci_ros::init("mobile");

    let speaker = Arc::new(PrintSpeaker::new());
    let move_base = Arc::new(RosCmdVelMoveBase::new("/cmd_vel"));
    let gamepad = Arc::new(GilGamepad::new_from_config(GilGamepadConfig::default()));

    let teleop_manager = TeleopManager::new(move_base, speaker, gamepad);

    loop {
        teleop_manager.update().await;
        teleop_manager.speak().await;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(not(feature = "ros"))]
fn main() {
    println!("ROS is not featured.");
}

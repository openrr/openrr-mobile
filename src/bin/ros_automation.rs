#[cfg(feature = "ros")]
fn main() {
    use arci_ros::RosCmdVelMoveBase;
    use openrr_mobile::robot_controller::RobotController;

    // Initialize
    arci_ros::init("mobile");

    let move_base = RosCmdVelMoveBase::new("/cmd_vel");

    let mut robot_controller = RobotController::new(move_base);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[cfg(not(feature = "ros"))]
fn main() {
    println!("ROS is not featured.");
}

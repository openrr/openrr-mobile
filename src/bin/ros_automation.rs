#[cfg(feature = "ros")]
fn main() {
    use arci::BaseVelocity;
    use arci_ros::RosCmdVelMoveBase;
    use openrr_mobile::robot_controller::RobotController;

    // Initialize
    arci_ros::init("mobile");

    let move_base = RosCmdVelMoveBase::new("/cmd_vel");

    let min = BaseVelocity::new(-1f64, -1f64, -1f64);
    let max = BaseVelocity::new(1f64, 1f64, 1f64);

    let mut robot_controller = RobotController::new(move_base, min, max);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[cfg(not(feature = "ros"))]
fn main() {
    println!("ROS is not featured.");
}

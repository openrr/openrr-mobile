#[cfg(feature = "ros2")]
fn main() {
    use arci::BaseVelocity;
    use arci_ros2::{r2r, Ros2CmdVelMoveBase};
    use openrr_mobile::robot_controller::RobotController;

    let ctx = r2r::Context::create().unwrap();

    let move_base = Ros2CmdVelMoveBase::new(ctx, "/cmd_vel");

    let min = BaseVelocity::new(-1f64, -1f64, -1f64);
    let max = BaseVelocity::new(1f64, 1f64, 1f64);

    let mut robot_controller = RobotController::new(move_base, min, max);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[cfg(not(feature = "ros2"))]
fn main() {
    println!("ROS2 is not featured.");
}

use arci::BaseVelocity;
use openrr_mobile::robot_controller::RobotController;

fn main() {
    #[allow(unused_mut)]
    let mut move_base_client: _;

    #[cfg(feature = "ros")]
    {
        use arci_ros::RosCmdVelMoveBase;

        arci_ros::init("mobile");
        move_base_client = RosCmdVelMoveBase::new("/cmd_vel");
    }

    #[cfg(feature = "ros2")]
    {
        use arci_ros2::{r2r, Ros2CmdVelMoveBase};

        let ctx = r2r::Context::create().unwrap();
        move_base_client = Ros2CmdVelMoveBase::new(ctx, "/cmd_vel");
    }

    #[cfg(not(any(feature = "ros", feature = "ros2")))]
    {
        use arci_urdf_viz::UrdfVizWebClient;

        move_base_client = UrdfVizWebClient::default();
        move_base_client.run_send_velocity_thread();
    }

    let min = BaseVelocity::new(-1f64, -1f64, -1f64);
    let max = BaseVelocity::new(1f64, 1f64, 1f64);

    let mut robot_controller = RobotController::new(move_base_client, min, max);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

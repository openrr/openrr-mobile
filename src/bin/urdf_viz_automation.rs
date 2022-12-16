use arci::BaseVelocity;
use arci_urdf_viz::UrdfVizWebClient;
use openrr_mobile::robot_controller::RobotController;

fn main() {
    let urdf_viz_client = UrdfVizWebClient::default();
    urdf_viz_client.run_send_velocity_thread();

    let min = BaseVelocity::new(-1f64, -1f64, -1f64);
    let max = BaseVelocity::new(1f64, 1f64, 1f64);

    let mut robot_controller = RobotController::new(urdf_viz_client, min, max);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

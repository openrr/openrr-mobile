use arci_urdf_viz::UrdfVizWebClient;
use openrr_mobile::robot_controller::RobotController;

fn main() {
    let urdf_viz_client = UrdfVizWebClient::default();
    urdf_viz_client.run_send_velocity_thread();

    let mut robot_controller = RobotController::new(urdf_viz_client);

    loop {
        robot_controller.random_walk();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

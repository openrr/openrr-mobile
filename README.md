# OpenRR Mobile

Mobile robot controller using `OpenRR`

## Binary apps list

| Binary name | File name                 | Description                               | Remarks                             |
| ----------- | ------------------------- | ----------------------------------------- | ----------------------------------- |
| teleop      | teleoperation.rs          | Control robot using gamepad.              | features `ros` (optional)           |
| auto        | automation.rs             | Robot does random walk.                   | features `ros` or `ros2` (optional) |
| uviz_teleop | urdf_viz_teleoperation.rs | Same as `teleop` but only for `urdf-viz`. |                                     |
| uviz_auto   | urdf_viz_automation.rs    | Same as `auto` but only for `urdf-viz`.   |                                     |
| ros_teleop  | ros_teleoperation.rs      | Same as `teleop` but only for `ROS`.      | features `ros` (required)           |
| ros_auto    | ros_automation.rs         | Same as `auto` but only for `ROS`.        | features `ros` (required)           |
| ros2_auto   | ros2_automation.rs        | Same as `auto` but only for `ROS2`.       | features `ros2` (required)          |

## Build and Run

```bash
cargo run --bin <Binary name> [optional features]
```

### Example (`urdf-viz`)

If current directory is `openrr`.

```bash
urdf-viz ./openrr-planner/sample.urdf &
cargo run --bin {teleop / auto / uviz_teleop / uviz_auto}
```

If you have used a `Turtlebot3`.

```bash
urdf-viz $(rospack find turtlebot3_description)/urdf/turtlebot3_burger.urdf.xacro &
cargo run --bin {teleop / auto / uviz_teleop / uviz_auto}
```

### Example (`ROS`)

Open new terminal

```bash
roslaunch turtlebot3_gazebo turtlebot3_empty_world.launch
```

```bash
rosrun joy joy_node
```

```bash
cargo run --features ros --bin {teleop / auto / ros_teleop / ros_auto}
```

### Example (`ROS2`)

```bash
ros2 launch turtlebot3_gazebo empty_world.launch.py
```

```bash
cargo run --features ros2 --bin {auto / ros2_auto}
```

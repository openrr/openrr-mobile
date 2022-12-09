use arci::{BaseVelocity, MoveBase};
use rand::Rng;

pub struct RobotController<M>
where
    M: MoveBase + 'static,
{
    move_base: M,
    velocity: BaseVelocity,
    min_velocity: BaseVelocity,
    max_velocity: BaseVelocity,

    state: State,
}

impl<M> RobotController<M>
where
    M: MoveBase + 'static,
{
    pub fn new(move_base: M) -> Self {
        Self {
            move_base,
            velocity: BaseVelocity::new(0f64, 0f64, 0f64),
            // TODO: Read from toml file
            min_velocity: BaseVelocity {
                x: -1f64,
                y: -1f64,
                theta: -1f64,
            },
            max_velocity: BaseVelocity {
                x: 1f64,
                y: 1f64,
                theta: 1f64,
            },

            state: State::ChangingDirection,
        }
    }

    pub fn current_velocity(&self) -> BaseVelocity {
        self.velocity
    }

    pub fn update(&mut self, idx: Index, value: f64) {
        match idx {
            Index::X => {
                if is_in_range(value, self.min_velocity.x, self.max_velocity.x) {
                    self.velocity.x = value;
                }
            }
            Index::Y => {
                if is_in_range(value, self.min_velocity.y, self.max_velocity.y) {
                    self.velocity.y = value;
                }
            }
            Index::Theta => {
                if is_in_range(value, self.min_velocity.theta, self.max_velocity.theta) {
                    self.velocity.theta = value;
                }
            }
        }
        self.move_base.send_velocity(&self.velocity).unwrap();
    }

    pub fn random_walk(&mut self) {
        match self.state {
            State::ChangingDirection => {
                self.velocity = BaseVelocity::new(0f64, 0f64, self.random_velocity(Index::Theta));
                self.state = State::Walking;
            }
            State::Walking => {
                self.velocity = BaseVelocity::new(
                    self.random_velocity(Index::X),
                    self.random_velocity(Index::Y),
                    0f64,
                );
                self.state = State::ChangingDirection;
            }
        }

        self.move_base.send_velocity(&self.velocity).unwrap();
    }

    fn random_velocity(&self, idx: Index) -> f64 {
        let mut rng = rand::thread_rng();
        match idx {
            Index::X => rng.gen_range(self.min_velocity.x..self.max_velocity.x),
            Index::Y => rng.gen_range(self.min_velocity.y..self.max_velocity.y),
            Index::Theta => rng.gen_range(self.min_velocity.theta..self.max_velocity.theta),
        }
    }
}

pub enum Index {
    X,
    Y,
    Theta,
}

pub enum State {
    ChangingDirection,
    Walking,
}

pub fn is_in_range(target: f64, min: f64, max: f64) -> bool {
    target > min && target < max
}

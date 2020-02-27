use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage},
};

use crate::catvolleyball::Ball;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

pub const GRAVITY_ACCELERATION: f32 = -40.0;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        // https://gamedev.stackexchange.com/questions/15708/how-can-i-implement-gravity
        for (ball, local) in (&mut balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(
                (ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION / 2.0)
                    * time.delta_seconds(),
            );
            ball.velocity[1] = ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}

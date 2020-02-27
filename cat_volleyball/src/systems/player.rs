use amethyst::{
    core::timing::Time,
    core::SystemDesc,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::catvolleyball::{Player, Side, ARENA_WIDTH, PLAYER_WIDTH};

const PLAYER_SPEED: f32 = 60.0;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, time, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let movement = match player.side {
                Side::Left => input.axis_value("left_player"),
                Side::Right => input.axis_value("right_player"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = PLAYER_SPEED * time.delta_seconds() * mv_amount as f32;
                let player_x = transform.translation().x;
                let player_left_limit = match player.side {
                    Side::Left => 0.0,
                    Side::Right => ARENA_WIDTH / 2.0,
                };
                transform.set_translation_x(
                    (player_x + scaled_amount)
                        .max(player_left_limit + PLAYER_WIDTH / 2.0)
                        .min(player_left_limit + ARENA_WIDTH / 2.0 - PLAYER_WIDTH / 2.0),
                );
            }
        }
    }
}

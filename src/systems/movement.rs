use bevy::prelude::*;
use crate::components::Player;

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    _time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += player.mv;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= player.mv;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= player.mv;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += player.mv;
        }
    }
}
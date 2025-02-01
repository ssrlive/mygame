use super::{BIRD_ANIMATION_SPEED, FALL_SPEED, FALL_VELOCITY_LIMIT, JUMP_AMOUNT, MOVE_SPEED};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Bird {
    velocity: f32,
}

pub(super) fn jump(mut bird: Query<&mut Bird>) {
    for mut bird in &mut bird {
        bird.velocity = JUMP_AMOUNT;
    }
}

pub(super) fn fall(mut bird: Query<&mut Bird, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        bird.velocity -= FALL_SPEED * time.delta_secs();
        bird.velocity = bird.velocity.max(FALL_VELOCITY_LIMIT);
    }
}

pub(super) fn move_bird(mut bird: Query<(&mut Transform, &Bird), With<Bird>>, time: Res<Time>) {
    for (mut transform, bird) in &mut bird {
        transform.translation.y += bird.velocity * MOVE_SPEED * time.delta_secs();
    }
}

pub(super) fn animate_bird(mut bird: Query<&mut Sprite, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        let texture_atlas = bird.texture_atlas.as_mut().unwrap();
        texture_atlas.index = (time.elapsed_secs() * BIRD_ANIMATION_SPEED) as usize % 4;
    }
}

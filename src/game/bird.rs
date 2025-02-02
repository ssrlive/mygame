use super::{BIRD_ANIMATION_SPEED, FALL_SPEED, FALL_VELOCITY_LIMIT, JUMP_AMOUNT, MOVE_SPEED};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Bird {
    velocity: f32,
}

pub(super) fn jump(mut query: Query<&mut Bird>) {
    for mut bird in &mut query {
        bird.velocity = JUMP_AMOUNT;
    }
}

pub(super) fn fall(mut query: Query<&mut Bird>, time: Res<Time>) {
    for mut bird in &mut query {
        bird.velocity -= FALL_SPEED * time.delta_secs();
        bird.velocity = bird.velocity.max(FALL_VELOCITY_LIMIT);
    }
}

pub(super) fn move_bird(mut query: Query<(&mut Transform, &Bird)>, time: Res<Time>) {
    for (mut transform, bird) in &mut query {
        transform.translation.y += bird.velocity * MOVE_SPEED * time.delta_secs();
    }
}

pub(super) fn animate_bird(mut query: Query<&mut Sprite, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut query {
        if let Some(texture_atlas) = bird.texture_atlas.as_mut() {
            texture_atlas.index = (time.elapsed_secs() * BIRD_ANIMATION_SPEED) as usize % 4;
        }
    }
}

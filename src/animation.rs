#![allow(dead_code)]

use bevy::prelude::*;
use std::collections::HashMap;

pub fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Animator, &mut Sprite)>) {
    for (mut animator, mut sprite) in query.iter_mut() {
        let anim = animator.animation_bank[animator.current_animation.as_str()];
        let Some(texture_atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animator.last_animation != animator.current_animation {
            texture_atlas.index = anim.start - 1;
        }
        animator.timer -= time.delta().as_secs_f32();
        if animator.timer <= 0. {
            animator.timer = anim.cooldown;
            if anim.looping {
                texture_atlas.index = ((texture_atlas.index + 1 - (anim.start - 1)) % (anim.end - anim.start + 1)) + anim.start - 1;
            } else {
                texture_atlas.index += 1;
                if texture_atlas.index > anim.end - 1 {
                    texture_atlas.index = anim.end - 1;
                }
            }
        }
        animator.last_animation = animator.current_animation.clone();
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Animation {
    pub start: usize,
    pub end: usize,
    pub looping: bool,
    pub cooldown: f32,
}

impl Animation {
    pub fn new(start: usize, end: usize, looping: bool, cooldown: f32) -> Self {
        Self {
            start,
            end,
            looping,
            cooldown,
        }
    }
}

#[derive(Clone, Component, Debug, Default)]
pub struct Animator {
    pub animation_bank: HashMap<String, Animation>,
    pub current_animation: String,
    pub last_animation: String,
    pub timer: f32,
    pub cooldown: f32,
}

impl Animator {
    pub fn new(animation_bank: HashMap<String, Animation>, current_animation: &str, timer: f32, cooldown: f32) -> Self {
        Self {
            animation_bank,
            last_animation: current_animation.to_string(),
            current_animation: current_animation.to_string(),
            timer,
            cooldown,
        }
    }
}

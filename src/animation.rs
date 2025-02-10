use bevy::prelude::*;

use crate::bird::PlayerTimer;

pub struct MyAnimationPlugin;

pub struct AnimationFrame {
    pub index: i32,
    pub time: f32,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame: i32,
}

#[derive(Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
    pub current_animation: i32,
}

impl Plugin for MyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_system);
    }
}

fn animate_system(mut query: Query<(&mut PlayerTimer, &mut Sprite, &mut Animations)>, time: Res<Time>) {
    let Ok((mut timer, mut sprite, mut animations)) = query.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    let current_animation_index = animations.current_animation;
    let Some(animation) = animations.animations.get_mut(current_animation_index as usize) else {
        return;
    };
    animation.current_frame += 1;
    if animation.current_frame as usize >= animation.frames.len() {
        animation.current_frame = 0;
    }
    let frame_data = animation.frames.get(animation.current_frame as usize).unwrap();
    let v = std::time::Duration::from_secs_f32(frame_data.time);
    timer.0.set_duration(v);
    timer.0.reset();

    if let Some(frame) = animation.frames.get(animation.current_frame as usize) {
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = frame.index as usize;
        }
    }
}

use bevy::prelude::*;

use crate::{ascii::AsciiSheet, GameState};

pub struct FadeoutPlugin;

#[derive(Component)]
struct ScreenFade {
    alpha: f32,
    sent: bool,
    next_state: GameState,
    timer: Timer,
}

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fadeout);
    }
}

fn fadeout(
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut Sprite)>,
    mut state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
        fade.timer.tick(time.delta());
        if fade.timer.fraction() < 0.5 {
            fade.alpha = fade.timer.fraction() * 2.0;
        } else {
            fade.alpha = fade.timer.fraction_remaining() * 2.0;
        }
        sprite.color.set_alpha(fade.alpha);

        if fade.timer.fraction() > 0.5 && !fade.sent {
            state.set(fade.next_state);
            fade.sent = true;
        }

        if fade.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_fadeout(commands: &mut Commands, next_state: GameState, ascii: &Res<AsciiSheet>) {
    let mut sprite = ascii.0.clone();
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = 0;
    }
    sprite.color = Color::srgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(100000.0));

    commands
        .spawn((
            sprite,
            Transform::from_translation(Vec3::new(0.0, 0.0, 999.0)),
        ))
        .insert(ScreenFade {
            alpha: 0.0,
            sent: false,
            next_state,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        })
        .insert(Name::new("Fadeout"));
}

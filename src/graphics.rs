use bevy::prelude::*;

pub struct GraphicsPlugin;

#[derive(Resource)]
pub struct CharacterSheet {
    handle: Sprite,
    bat_frames: [usize; 3],
}

#[derive(Component)]
pub struct FrameAnimation {
    timer: Timer,
    frames: Vec<usize>,
    current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::load_graphics)
            .add_systems(Update, Self::frame_animation);
    }
}

pub fn spawn_bat_sprite(
    commands: &mut Commands,
    characters: &CharacterSheet,
    translation: Vec3,
) -> Entity {
    let mut sprite = characters.handle.clone();
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = characters.bat_frames[0];
    }
    sprite.custom_size = Some(Vec2::splat(0.5));

    commands
        .spawn((sprite, Transform::from_translation(translation)))
        .insert(FrameAnimation {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            frames: characters.bat_frames.to_vec(),
            current_frame: 0,
        })
        .id()
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let atlas =
            TextureAtlasLayout::from_grid(UVec2::splat(16), 12, 8, Some(UVec2::splat(2)), None);
        let atlas_handle = texture_atlases.add(atlas);
        let sprite = Sprite::from_atlas_image(assets.load("characters.png"), atlas_handle.into());
        commands.insert_resource(CharacterSheet {
            handle: sprite,
            bat_frames: [12 * 4 + 3, 12 * 4 + 4, 12 * 4 + 5],
        });
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut Sprite, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation.frames[animation.current_frame];
                }
            }
        }
    }
}

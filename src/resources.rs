use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W as _, TILE_H as _),
        SPRITE_SHEET_W as _,
        SPRITE_SHEET_H as _,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::MainMenu);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate());
}

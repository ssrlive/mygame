use bevy::prelude::*;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
}

impl PlayerAttach {
    pub fn new(offset: Vec2) -> Self {
        Self { offset }
    }
}

pub fn attach_objects(
    player_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerAttach>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerMovement>>,
) {
    if let Ok((_movement_data, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            transform.translation = player_transform.translation + Vec3::new(attach.offset.x, attach.offset.y, 0.0);
        }
    }
}

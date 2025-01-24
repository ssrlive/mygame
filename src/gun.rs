use bevy::{prelude::*, window::PrimaryWindow};

use crate::cursor_info::OffsetedCursorPositon;

#[derive(Component)]
pub struct GunController;

pub fn gun_controls(
    mut cursor_res: ResMut<OffsetedCursorPositon>,
    mut gun_query: Query<(&GunController, &mut Transform)>,
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (_gun_controller, mut transform) in gun_query.iter_mut() {
        let Ok(primary) = primary_query.get_single() else {
            return;
        };
        let cursor_position = match cursor.read().last() {
            Some(cursor_moved) => {
                Vec2::new(cursor_moved.position.x, -cursor_moved.position.y) + Vec2::new(-primary.width() / 2.0, primary.height() / 2.0)
            }
            None => cursor_res.0,
        };

        cursor_res.0 = cursor_position;

        let diff = cursor_position - transform.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
    }
}

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct GunController;

pub fn gun_controls(
    mut gun_query: Query<(&GunController, &mut Transform)>,
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (_gun_controller, mut transform) in gun_query.iter_mut() {
        let Ok(primary) = primary_query.get_single() else {
            return;
        };
        let mut cursor_position = match cursor.read().last() {
            Some(cursor_moved) => cursor_moved.position,
            None => return,
        };
        cursor_position.x -= primary.width() / 2.0;
        cursor_position.y -= primary.height() / 2.0;

        let diff = cursor_position - Vec2::new(transform.translation.x, transform.translation.y);
        let angle = -diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
    }
}

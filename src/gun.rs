use bevy::{prelude::*, window::PrimaryWindow};

use crate::{animation::Animator, bullet::BulletParameters, cursor_info::OffsetedCursorPositon};

const BULLET_LIFETIME: f32 = 10.0;
const BULLET_SPEED: f32 = 1000.0;

#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
}

impl GunController {
    pub fn new(shoot_cooldown: f32) -> Self {
        Self {
            shoot_cooldown,
            shoot_timer: 0.0,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn gun_controls(
    mut cursor_res: ResMut<OffsetedCursorPositon>,
    mut gun_query: Query<(&mut GunController, &mut Transform, &mut Animator)>,
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (mut gun_controller, mut transform, mut animator) in gun_query.iter_mut() {
        gun_controller.shoot_timer -= time.delta_secs();
        animator.current_animation = if gun_controller.shoot_timer > 0.0 { "Shoot" } else { "Idle" }.to_string();
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

        if gun_controller.shoot_timer <= 0.0 && buttons.pressed(MouseButton::Left) {
            gun_controller.shoot_timer = gun_controller.shoot_cooldown;

            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.0));
            spawn_transform.translation = transform.translation;
            spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);

            let bullet_sprite = Sprite::from_image(asset_server.load("bullet.png"));
            let bullet_param = BulletParameters::new(BULLET_LIFETIME, BULLET_SPEED, diff.normalize());
            commands.spawn((bullet_sprite, spawn_transform, bullet_param));
        }
    }
}

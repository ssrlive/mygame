use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::ReadDefaultRapierContext, prelude::QueryFilter};

use super::{camera_controller::CameraController, player_plugin::Player};
use crate::game::{
    level::targets::{DeadTarget, Target},
    shooting::tracer::BulletTracer,
};

#[derive(Component)]
pub struct Shootable;

#[derive(Component)]
pub struct TracerSpawnSpot;

#[allow(clippy::too_many_arguments)]
pub fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: ReadDefaultRapierContext,
    mut query: Query<(&mut Player, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraController>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Option<&Target>, With<Shootable>>,
    spawn_spot: Query<&GlobalTransform, With<TracerSpawnSpot>>,
) {
    let spawn_spot = spawn_spot.get_single().unwrap();
    let window = window_query.get_single().unwrap();
    let Ok((_, _transform)) = query.get_single_mut() else {
        return;
    };
    let Ok((camera, global_transform)) = camera_query.get_single() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let viewport_position = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        let Ok(ray) = camera.viewport_to_world(global_transform, viewport_position) else {
            return;
        };
        let predicate = |entity| target_query.get(entity).is_ok();
        let query_filter = QueryFilter::new().predicate(&predicate);
        let hit = rapier_context.cast_ray_and_get_normal(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            query_filter,
        );
        if let Some((entity, ray_intersection)) = hit {
            if let Ok(target) = target_query.get(entity) {
                if target.is_some() {
                    commands.entity(entity).insert(DeadTarget);
                }
            }
            // spawn tracer and check collisions
            let tracer_material = StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 0.0),
                unlit: true,
                ..default()
            };
            commands.spawn((
                Transform::from_translation(Vec3::splat(f32::MAX)),
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.0)))),
                MeshMaterial3d(materials.add(tracer_material)),
                BulletTracer::new(spawn_spot.translation(), ray_intersection.point, 100.0),
            ));
        }
    }
}

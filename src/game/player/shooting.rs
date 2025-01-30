use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::ReadDefaultRapierContext, prelude::QueryFilter};

use super::player::Player;
use crate::game::{
    level::targets::{DeadTarget, Target},
    shooting::tracer::BulletTracer,
};

pub fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: ReadDefaultRapierContext,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Entity, With<Target>>,
) {
    let window = window_query.get_single().unwrap();
    let Ok((_, transform, global_transform, camera)) = query.get_single_mut() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let viewport_position = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        let Ok(ray) = camera.viewport_to_world(&global_transform, viewport_position) else {
            return;
        };
        let hit = rapier_context.cast_ray_and_get_normal(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            QueryFilter::default(),
        );
        if let Some((entity, ray_intersection)) = hit {
            if let Ok(_entity) = target_query.get(entity) {
                commands.entity(entity).insert(DeadTarget);
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
                BulletTracer::new(transform.translation, ray_intersection.point, 100.0),
            ));
        }
    }
}

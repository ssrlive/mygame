use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{
    plugin::ReadDefaultRapierContext,
    prelude::{CharacterLength, Collider, KinematicCharacterController, QueryFilter, RigidBody},
};

use super::{
    camera_controller::{update_camera_controller, CameraController},
    input::PlayerInput,
    movement::{update_movement, update_movement_input},
};
use crate::game::{
    level::targets::{DeadTarget, Target},
    shooting::tracer::{BulletTracer, TracerPlugin},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (
                    update_movement_input,
                    update_player,
                    update_camera_controller,
                ),
            )
            .add_systems(FixedUpdate, update_movement); // physics timestep
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(velocity: Vec3, gravity: f32, speed: f32) -> Self {
        Self {
            velocity,
            gravity,
            speed,
        }
    }
}

fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();
    let camera_entity = commands
        .spawn((
            Transform::IDENTITY,
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection { fov, ..default() }),
            CameraController {
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
                sensitivity: 0.035,
            },
        ))
        .id();
    let player_entity = commands
        .spawn((
            Player::new(Vec3::ZERO, 9.81, 20.0),
            Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            Visibility::default(),
            Collider::cuboid(1.0, 10.0, 1.0),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
        ))
        .id();
    commands.entity(player_entity).add_child(camera_entity);
}

fn update_player(
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

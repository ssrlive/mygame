use std::f32::consts::PI;

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rapier3d::prelude::Collider;
use rand::{rngs::ThreadRng, Rng};

pub struct TargetsPlugin;

impl Plugin for TargetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_grid_shot)
            .add_systems(Update, update_targets);
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct DeadTarget;

#[derive(Resource, Clone, Copy)]
pub struct GridShot {
    pub grid_size: i32,
    pub cell_size: f32,
    pub max_targets: i32,
}

impl GridShot {
    pub fn generate_new_position(&self, rng: &mut ThreadRng) -> Vec2 {
        let x = rng.random_range(0..self.grid_size) as f32;
        let y = rng.random_range(0..self.grid_size) as f32;
        let v2 = Vec2::new(self.grid_size as f32 / 2.0, 0.);
        (Vec2::new(x, y) - v2 + Vec2::Y * 0.5) * self.cell_size
    }
}

fn init_grid_shot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_shot = GridShot {
        grid_size: 5,
        cell_size: 5.0,
        max_targets: 5,
    };
    let target_material = materials.add(StandardMaterial {
        base_color: RED.into(),
        ..default()
    });
    commands.insert_resource(grid_shot);

    let target_radius = grid_shot.cell_size / 8.0;
    let collider_radius = target_radius * f32::sin(PI / 4.);
    for _ in 0..grid_shot.max_targets {
        commands.spawn((
            Target,
            DeadTarget,
            Collider::cuboid(collider_radius, collider_radius, collider_radius),
            Transform::from_xyz(0.0, 0.0, -40.0),
            Mesh3d(meshes.add(Sphere::new(target_radius))),
            MeshMaterial3d(target_material.clone()),
        ));
    }
}

fn update_targets(
    mut commands: Commands,
    grid_shot: Res<GridShot>,
    mut dead_targets: Query<(Entity, &mut Transform), (With<Target>, With<DeadTarget>)>,
    alive_targets: Query<&Transform, (With<Target>, Without<DeadTarget>)>,
) {
    let mut alive_target_positions = Vec::new();
    let mut rng = rand::rng();
    for transform in alive_targets.iter() {
        alive_target_positions.push(transform.translation.truncate());
    }
    for (entity, mut transform) in dead_targets.iter_mut() {
        let mut found_spot = false;
        let old_position = transform.translation.truncate();
        let mut new_position = grid_shot.generate_new_position(&mut rng);
        while !found_spot {
            found_spot = true;
            // make sure we don't reset to same position
            while new_position == old_position {
                new_position = grid_shot.generate_new_position(&mut rng);
            }
            for position in &alive_target_positions {
                if *position == new_position {
                    found_spot = false;
                    new_position = grid_shot.generate_new_position(&mut rng);
                    break;
                }
            }
        }
        commands.entity(entity).remove::<DeadTarget>();
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
        alive_target_positions.push(new_position);
    }
}

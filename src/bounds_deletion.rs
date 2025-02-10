use bevy::prelude::*;

#[derive(Component)]
pub struct OffsceenDeletion;

pub struct BoundsDeletionPlugin;

impl Plugin for BoundsDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, offscreen_remove_system);
    }
}

fn offscreen_remove_system(mut commands: Commands, pipe_query: Query<(Entity, &Transform, &OffsceenDeletion)>) {
    let padding = 300.0;
    for (entity, transform, _od) in &mut pipe_query.iter() {
        // Left side of screen
        if transform.translation.x < -1920.0 * 0.5 - padding {
            commands.entity(entity).despawn_recursive();
        }
    }
}

use bevy::prelude::*;
use itertools::Itertools;
use rand::{distributions::WeightedIndex, prelude::Distribution};
pub mod position;
use position::*;

use crate::{assets::ImageAssets, colors};

pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACER: f32 = 0.0;

#[derive(Resource)]
pub struct Board {
    pub size: u16,
    physical_size: f32,
}

impl Board {
    pub fn new(size: u16) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board { size, physical_size }
    }
    pub fn cell_position_to_physical(&self, pos: i32) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + pos as f32 * TILE_SIZE + (pos + 1) as f32 * TILE_SPACER
    }
    pub fn low_edge(&self) -> f32 {
        -self.physical_size / 2.0
    }
    pub fn high_edge(&self) -> f32 {
        self.physical_size / 2.0
    }
    pub fn tiles(&self) -> impl Iterator<Item = Position> {
        (0..self.size)
            .cartesian_product(0..self.size)
            .map(|(x, y)| Position(IVec2::new(i32::from(x), i32::from(y))))
    }
}

pub fn spawn_board(mut commands: Commands, images: Res<ImageAssets>, board: Res<Board>) {
    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

    commands
        .spawn(Sprite::from_color(colors::BOARD, Vec2::splat(board.physical_size)))
        .with_children(|builder| {
            for pos in board.tiles() {
                let mut texture_atlas = TextureAtlas::from(images.grass_layout.clone());
                texture_atlas.index = dist.sample(&mut rng);
                let mut sprite = Sprite::from_atlas_image(images.grass.clone(), texture_atlas);
                sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
                builder.spawn((
                    sprite,
                    Transform::from_xyz(board.cell_position_to_physical(pos.x), board.cell_position_to_physical(pos.y), 1.0),
                ));
            }
        });
}

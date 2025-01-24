use bevy::prelude::*;

#[derive(Resource, Clone, Copy, PartialEq, Debug)]
pub struct OffsetedCursorPositon(pub Vec2);

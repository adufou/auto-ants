use bevy::prelude::*;

/// The actual current direction the human is facing
/// This will smoothly rotate toward the desired direction
#[derive(Component, Default, Debug, Clone)]
pub struct CurrentDirection {
    pub direction: Vec2,
}

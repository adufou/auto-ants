use bevy::prelude::*;

/// The desired direction calculated from all movement influences
/// This is what the human WANTS to face (before rotation constraints)
#[derive(Component, Default, Debug, Clone)]
pub struct DesiredDirection {
    pub direction: Vec2,
}

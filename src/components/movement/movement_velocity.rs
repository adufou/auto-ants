use bevy::prelude::*;

/// Component that stores the final combined velocity for an entity
/// This velocity is calculated by combining all movement influences
/// and is applied to the Transform each frame
#[derive(Component, Default, Debug, Clone)]
pub struct MovementVelocity {
    pub velocity: Vec2,
}

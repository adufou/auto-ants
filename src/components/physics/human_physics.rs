use avian2d::prelude::*;
use bevy::prelude::*;

use crate::resources::PhysicsConfig;

/// Bundle containing all physics components for a human entity
/// Uses configuration from PhysicsConfig resource to set up Avian physics
#[derive(Bundle)]
pub struct HumanPhysics {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub locked_axes: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub max_linear_speed: MaxLinearSpeed,
}

impl HumanPhysics {
    /// Creates a new HumanPhysics bundle using values from PhysicsConfig
    pub fn new(config: &PhysicsConfig) -> Self {
        let physics = &config.human;

        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(physics.collider_radius),
            mass: Mass(physics.mass),
            linear_velocity: LinearVelocity::ZERO,
            linear_damping: LinearDamping(physics.linear_damping),
            angular_damping: AngularDamping(physics.angular_damping),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::new(physics.friction),
            restitution: Restitution::new(physics.restitution),
            max_linear_speed: MaxLinearSpeed(physics.max_linear_speed),
        }
    }
}

use bevy::prelude::*;

/// Collider radius in pixels (sprites are 16x16, use radius 7 for ~14px diameter)
const HUMAN_COLLIDER_RADIUS: f32 = 8.0;

/// Mass for each human in physics simulation (affects inertia)
const HUMAN_MASS: f32 = 1.0;

/// Linear damping (prevents infinite acceleration, works with forces to reach equilibrium)
const HUMAN_DAMPING: f32 = 2.0;

/// Angular damping (reduces rotation over time)
const HUMAN_ANGULAR_DAMPING: f32 = 2.0;

/// Maximum linear speed in pixels per second (matches base_speed from MovementConfig)
const MAX_LINEAR_SPEED: f32 = 50.0;

/// Configuration for physics simulation properties across entity types
/// Centralizes physics constants for consistency and easy tuning
#[derive(Resource, Debug, Clone)]
pub struct PhysicsConfig {
    /// Human-specific physics configuration
    pub human: HumanPhysicsConfig,
    // Future: Add other entity types here
    // pub enemy: EnemyPhysicsConfig,
    // pub object: ObjectPhysicsConfig,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            human: HumanPhysicsConfig::default(),
        }
    }
}

/// Physics properties specific to human entities
#[derive(Debug, Clone)]
pub struct HumanPhysicsConfig {
    /// Collider radius in pixels (sprites are 16x16, radius 7 ≈ 14px diameter)
    pub collider_radius: f32,

    /// Mass for physics simulation (affects inertia and collision response)
    pub mass: f32,

    /// Linear damping (prevents infinite acceleration, creates natural deceleration)
    pub linear_damping: f32,

    /// Angular damping (reduces rotation over time)
    pub angular_damping: f32,

    /// Maximum linear speed in pixels per second
    /// Should match MovementConfig.base_speed
    pub max_linear_speed: f32,

    /// Coefficient of friction (0.0 = frictionless sliding)
    pub friction: f32,

    /// Coefficient of restitution (bounciness, 0.0 = no bounce)
    pub restitution: f32,
}

impl Default for HumanPhysicsConfig {
    fn default() -> Self {
        Self {
            collider_radius: HUMAN_COLLIDER_RADIUS,
            mass: HUMAN_MASS,
            linear_damping: HUMAN_DAMPING,
            angular_damping: HUMAN_ANGULAR_DAMPING,
            max_linear_speed: MAX_LINEAR_SPEED,
            friction: 0.0,
            restitution: 0.0,
        }
    }
}

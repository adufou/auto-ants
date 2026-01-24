use crate::components::{CohesionInfluence, CurrentDirection, DesiredDirection, Human};
use crate::resources::DebugConfig;
use bevy::prelude::*;

// Visual styling constants
const VISION_CIRCLE_COLOR: Color = Color::srgba(0.3, 0.7, 1.0, 0.4); // Light blue, semi-transparent
const CURRENT_DIR_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green
const DESIRED_DIR_COLOR: Color = Color::srgb(1.0, 0.5, 0.0); // Orange
const ARROW_LENGTH: f32 = 50.0; // Length of direction arrows in pixels

/// System that draws debug visualizations for human vision and movement
/// Only runs when debug mode is enabled via the checkbox
pub fn vision_debug_gizmos(
    debug_config: Res<DebugConfig>,
    query: Query<
        (
            &Transform,
            &CurrentDirection,
            &DesiredDirection,
            Option<&CohesionInfluence>,
        ),
        With<Human>,
    >,
    mut gizmos: Gizmos,
) {
    // Early return if debug mode is disabled
    if !debug_config.show_vision_debug {
        return;
    }

    for (transform, current_dir, desired_dir, cohesion_opt) in &query {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);

        // Draw vision range circle (perception radius)
        if let Some(cohesion) = cohesion_opt {
            gizmos.circle_2d(pos, cohesion.perception_radius, VISION_CIRCLE_COLOR);
        }

        // Draw current direction arrow (green)
        if current_dir.direction.length_squared() > 0.001 {
            let end = pos + current_dir.direction.normalize() * ARROW_LENGTH;
            gizmos.arrow_2d(pos, end, CURRENT_DIR_COLOR);
        }

        // Draw desired direction arrow (orange)
        if desired_dir.direction.length_squared() > 0.001 {
            let end = pos + desired_dir.direction.normalize() * ARROW_LENGTH;
            gizmos.arrow_2d(pos, end, DESIRED_DIR_COLOR);
        }
    }
}

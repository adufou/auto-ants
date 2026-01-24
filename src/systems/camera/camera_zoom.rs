use crate::resources::CameraConfig;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

/// System to handle camera zoom via mouse wheel
/// Modifies the OrthographicProjection scale based on scroll input
pub fn camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    config: Res<CameraConfig>,
    mut camera_query: Query<&mut Projection, With<Camera2d>>,
) {
    // Sum up all scroll events this frame (for smooth trackpad scrolling)
    let scroll_delta: f32 = scroll_events.read().map(|ev| ev.y).sum();

    if scroll_delta != 0.0 {
        for mut projection in &mut camera_query {
            if let Projection::Orthographic(ortho) = projection.as_mut() {
                // Calculate new scale
                // Negative scroll = zoom in (decrease scale)
                // Positive scroll = zoom out (increase scale)
                let scale_change = -scroll_delta * config.zoom_sensitivity;
                let new_scale = ortho.scale + scale_change;

                // Clamp to configured limits
                ortho.scale = new_scale.clamp(config.min_scale, config.max_scale);
            }
        }
    }
}

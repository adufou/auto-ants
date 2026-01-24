use bevy::prelude::*;

/// Configuration for camera controls and behavior
#[derive(Resource, Debug, Clone)]
pub struct CameraConfig {
    /// Zoom sensitivity (scale change per scroll wheel unit)
    pub zoom_sensitivity: f32,

    /// Minimum scale value (maximum zoom in)
    pub min_scale: f32,

    /// Maximum scale value (maximum zoom out)
    pub max_scale: f32,

    /// Camera movement speed in pixels per second
    pub movement_speed: f32,

    /// Camera follow smoothing factor (higher = faster following, range 0.0-1.0)
    /// Recommended: 0.15 for smooth, responsive following
    pub follow_smoothing: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            zoom_sensitivity: 0.1,
            min_scale: 0.25,
            max_scale: 4.0,
            movement_speed: 200.0,
            follow_smoothing: 0.15,
        }
    }
}

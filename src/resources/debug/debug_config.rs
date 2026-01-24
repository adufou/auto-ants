use bevy::prelude::*;

/// Resource that holds global debug visualization state
#[derive(Resource, Default, Debug, Clone)]
pub struct DebugConfig {
    /// Whether to show vision range and direction debug visualizations
    pub show_vision_debug: bool,
}

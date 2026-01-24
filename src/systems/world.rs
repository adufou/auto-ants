pub mod setup;
pub mod camera;
pub mod movement;
pub mod spatial;
pub mod debug;

// Re-export all system functions
pub use setup::{setup_tilemap, setup_entities, generate_world};
pub use camera::{camera_controls, camera_zoom, track_camera_chunk};
pub use movement::{calculate_random_walk, resolve_movement, apply_movement, spawn_human};
pub use spatial::update_spatial_grid;
pub use debug::{terrain_tuning, performance_monitor};

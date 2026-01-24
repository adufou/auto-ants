pub mod movement;
pub mod setup;
pub mod spatial;

// Re-export all system functions
pub use movement::{apply_movement, calculate_random_walk, resolve_movement, spawn_human};
pub use setup::{generate_world, setup_entities, setup_tilemap};
pub use spatial::update_spatial_grid;

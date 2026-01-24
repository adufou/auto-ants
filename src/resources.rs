pub mod camera;
pub mod terrain;
pub mod movement;
pub mod rendering;
pub mod spatial;

// Re-export all public items
pub use camera::CameraConfig;
pub use terrain::{TerrainConfig, ChunkManager, ChunkCoord};
pub use movement::MovementConfig;
pub use rendering::{TilemapAssets, EntityAssets};
pub use spatial::SpatialGrid;

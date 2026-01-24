pub mod camera;
pub mod movement;
pub mod rendering;
pub mod spatial;
pub mod terrain;

// Re-export all public items
pub use camera::CameraConfig;
pub use movement::MovementConfig;
pub use rendering::{EntityAssets, TilemapAssets};
pub use spatial::SpatialGrid;
pub use terrain::{ChunkCoord, ChunkManager, TerrainConfig};

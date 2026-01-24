pub mod capabilities;
pub mod debug_ui;
pub mod debug_ui_systems;
pub mod test_floating_window;
pub mod test_floating_window_systems;

pub use capabilities::CapsUi;
pub use debug_ui::DebugUiRoot;
pub use debug_ui_systems::handle_checkbox_interaction;
pub use test_floating_window::TestFloatingWindowRoot;
pub use test_floating_window_systems::{handle_test_window_close, spawn_test_window};

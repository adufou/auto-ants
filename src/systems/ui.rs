pub mod debug_ui;
pub use debug_ui::{handle_checkbox_interaction, spawn_debug_ui};

pub mod human_info_window;
pub use human_info_window::{
    handle_human_info_close, spawn_human_info_window, update_human_info_content,
    validate_selected_human,
};

pub mod human_selection;
pub use human_selection::handle_human_click;

pub mod test_floating_window;
pub use test_floating_window::{handle_test_window_close, spawn_test_window};

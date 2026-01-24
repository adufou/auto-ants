use bevy::prelude::*;

// Re-export CapsUi from bevy_immediate
pub use bevy_immediate::ui::CapsUi;

// UI styling constants
pub const CHECKBOX_BG: Color = Color::srgba(0.2, 0.2, 0.2, 0.8);
pub const CHECKBOX_BORDER: Color = Color::WHITE;
pub const TEXT_COLOR: Color = Color::WHITE;
pub const CHECKBOX_SIZE: f32 = 20.0;
pub const CHECKBOX_BORDER_WIDTH: f32 = 2.0;
pub const UI_PADDING: f32 = 16.0;
pub const CHECKBOX_LABEL_GAP: f32 = 8.0;
pub const FONT_SIZE_NORMAL: f32 = 16.0;

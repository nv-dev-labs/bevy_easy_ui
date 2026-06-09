pub use crate::core::container::{Container, WithObservers};
pub use crate::core::image_node::EasyImageNode;
pub use crate::core::node::EasyNode;
pub use crate::core::parts::{box_style::*, stack_style::*, text_style::*};

pub use crate::widgets::containers::button::*;
pub use crate::widgets::containers::horizontal_layout::*;
pub use crate::widgets::containers::rich_text::*;
pub use crate::widgets::containers::vertical_layout::*;

pub use crate::widgets::image::*;
pub use crate::widgets::label::*;
pub use crate::widgets::span::*;
pub use crate::widgets::text::*;
pub use crate::widgets::text_input::*;
pub use crate::widgets::viewport::*;

// Re-exports to make it easier for users
pub use bevy::color::palettes::css::*;
pub use bevy::text::FontFeatures;
pub use bevy::text::FontSmoothing;

pub use bevy_ui_text_input::TextInputPlugin;

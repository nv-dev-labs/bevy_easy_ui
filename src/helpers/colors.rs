use bevy::color::Color;

pub struct EasyColor;

#[allow(dead_code)]
/// A collection of predefined colors and a helper function to create custom colors.
impl EasyColor {
  pub const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

  pub const WHITE: Color = Color::WHITE;
  pub const LIGHT_GRAY: Color = Color::srgba(0.75, 0.75, 0.75, 1.0);
  pub const GRAY: Color = Color::srgba(0.5, 0.5, 0.5, 1.0);
  pub const DARK_GRAY: Color = Color::srgba(0.25, 0.25, 0.25, 1.0);
  pub const BLACK: Color = Color::BLACK;

  pub const RED: Color = Color::srgba(1.0, 0.0, 0.0, 1.0);
  pub const GREEN: Color = Color::srgba(0.0, 1.0, 0.0, 1.0);
  pub const BLUE: Color = Color::srgba(0.0, 0.0, 1.0, 1.0);

  pub const YELLOW: Color = Color::srgba(1.0, 1.0, 0.0, 1.0);
  pub const CYAN: Color = Color::srgba(0.0, 1.0, 1.0, 1.0);
  pub const MAGENTA: Color = Color::srgba(1.0, 0.0, 1.0, 1.0);

  pub const ORANGE: Color = Color::srgba(1.0, 0.5, 0.0, 1.0);
  pub const PURPLE: Color = Color::srgba(0.5, 0.0, 0.5, 1.0);
  pub const BROWN: Color = Color::srgba(0.6, 0.3, 0.0, 1.0);
  pub const PINK: Color = Color::srgba(1.0, 0.75, 0.8, 1.0);

  pub const LIGHT_BLUE: Color = Color::srgba(0.68, 0.85, 0.9, 1.0);
  pub const DARK_BLUE: Color = Color::srgba(0.0, 0.0, 0.5, 1.0);
  pub const LIGHT_GREEN: Color = Color::srgba(0.56, 0.93, 0.56, 1.0);
  pub const DARK_GREEN: Color = Color::srgba(0.0, 0.5, 0.0, 1.0);
  pub const LIGHT_RED: Color = Color::srgba(1.0, 0.71, 0.76, 1.0);
  pub const DARK_RED: Color = Color::srgba(0.5, 0.0, 0.0, 1.0);

  pub const LIGHT_YELLOW: Color = Color::srgba(1.0, 1.0, 0.88, 1.0);
  pub const DARK_YELLOW: Color = Color::srgba(0.5, 0.5, 0.0, 1.0);
  pub const LIGHT_CYAN: Color = Color::srgba(0.68, 1.0, 1.0, 1.0);
  pub const DARK_CYAN: Color = Color::srgba(0.0, 0.5, 0.5, 1.0);
  pub const LIGHT_MAGENTA: Color = Color::srgba(1.0, 0.68, 1.0, 1.0);
  pub const DARK_MAGENTA: Color = Color::srgba(0.5, 0.0, 0.5, 1.0);

  pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::srgba(r, g, b, a)
  }
}

use bevy::{
  asset::Handle,
  image::Image,
  math::Vec2,
  prelude::*,
  text::{
    Font, FontFeatures, FontSmoothing, FontWeight, Justify, LineBreak, LineHeight, TextColor,
    TextFont, TextLayout,
  },
  ui::widget::TextShadow,
};

use crate::helpers::colors::EasyColor;

//>===========================================================================
// BUNDLE PARTS
//>===========================================================================
//
// Bevy 0.18's `#[derive(Bundle)]` automatically flattens any field that itself
// implements `Bundle`. So a widget can simply embed these structs as fields
// — no attribute needed.
//
// IMPORTANT: Bevy's `Text` already requires `TextFont`, `TextColor`,
// `TextLayout`, `LineHeight` via `#[require(...)]`. So `EasyTextProps`
// cannot include them (would create duplicates when flattened alongside `Text`).
// It only contains `TextShadow` (not required by `Text`).

// === Common visual styling (background, border, shadow) ===
// Used by all widgets (text or not).
#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyBoxStyle {
  pub background_color: BackgroundColor,
  pub border_color: BorderColor,
  pub border_gradient: BorderGradient,
  pub background_gradient: BackgroundGradient,
  pub box_shadow: BoxShadow,
  pub outline: Outline,
}

#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyStackStyle {
  pub z_index: ZIndex,
  pub global_z_index: GlobalZIndex,
}

// === Text-specific extra props (only what Text does NOT require) ===
// Used by text-bearing widgets (Text, Label, Span, RichText).
#[derive(Bundle, Debug, Clone)]
pub struct EasyTextProps {
  pub text_shadow: TextShadow,
  pub text_font: TextFont,
  pub text_color: TextColor,
  pub text_layout: TextLayout,
  pub line_height: LineHeight,
}

impl Default for EasyTextProps {
  fn default() -> Self {
    Self {
      text_shadow: TextShadow {
        color: EasyColor::TRANSPARENT,
        offset: Vec2::ZERO,
      },
      text_font: TextFont::default(),
      text_color: TextColor::default(),
      text_layout: TextLayout::default(),
      line_height: LineHeight::default(),
    }
  }
}

//>===========================================================================
// EXTENSION TRAITS (setters, defined once, applied via blanket impl)
//>===========================================================================

pub trait EasyStackStyleExt: Sized {
  fn stack_style_mut(&mut self) -> &mut EasyStackStyle;

  fn with_z_index(mut self, z_index: i32) -> Self {
    self.stack_style_mut().z_index = ZIndex(z_index);
    self
  }

  fn with_global_z_index(mut self, global_z_index: i32) -> Self {
    self.stack_style_mut().global_z_index = GlobalZIndex(global_z_index);
    self
  }
}

pub trait EasyBoxStyleExt: Sized {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle;

  fn with_background_color(mut self, color: Color) -> Self {
    self.easy_style_mut().background_color = BackgroundColor(color);
    self
  }

  fn with_border_color(mut self, color: Color) -> Self {
    self.easy_style_mut().border_color = BorderColor::all(color);
    self
  }

  fn with_box_shadow(mut self, box_shadow: BoxShadow) -> Self {
    self.easy_style_mut().box_shadow = box_shadow;
    self
  }

  fn with_border_gradient(mut self, border_gradient: BorderGradient) -> Self {
    self.easy_style_mut().border_gradient = border_gradient;
    self
  }

  fn with_background_gradient(mut self, background_gradient: BackgroundGradient) -> Self {
    self.easy_style_mut().background_gradient = background_gradient;
    self
  }

  fn with_outline(mut self, width: Val, offset: Val, color: Color) -> Self {
    self.easy_style_mut().outline = Outline {
      width,
      offset,
      color,
    };
    self
  }
}

// === EasyTextPropsExt (for text-bearing widgets) ===
pub trait EasyTextPropsExt: Sized {
  fn easy_props_mut(&mut self) -> &mut EasyTextProps;

  fn with_color(mut self, text_color: Color) -> Self {
    self.easy_props_mut().text_color = TextColor(text_color);
    self
  }

  fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
    self.easy_props_mut().text_shadow = text_shadow;
    self
  }

  fn with_shadow(mut self, text_shadow: TextShadow) -> Self {
    self.easy_props_mut().text_shadow = text_shadow;
    self
  }

  fn with_font_family(mut self, font: Handle<Font>) -> Self {
    self.easy_props_mut().text_font.font = font;
    self
  }

  fn with_font_size(mut self, font_size: f32) -> Self {
    self.easy_props_mut().text_font.font_size = font_size;
    self
  }

  fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
    self.easy_props_mut().text_font.weight = font_weight;
    self
  }

  fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
    self.easy_props_mut().text_font.font_smoothing = font_smoothing;
    self
  }

  fn with_features(mut self, font_features: FontFeatures) -> Self {
    self.easy_props_mut().text_font.font_features = font_features;
    self
  }
  fn with_justify(mut self, justify: Justify) -> Self {
    self.easy_props_mut().text_layout.justify = justify;
    self
  }

  fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
    self.easy_props_mut().text_layout.linebreak = linebreak;
    self
  }

  fn with_line_height(mut self, line_height: LineHeight) -> Self {
    self.easy_props_mut().line_height = line_height;
    self
  }
}

// Suppress unused import warnings for items used in trait signatures.
#[allow(dead_code)]
fn _unused_imports(_: Image, _: Vec2) {}

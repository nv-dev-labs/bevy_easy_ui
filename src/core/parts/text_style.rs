use bevy::{prelude::*, text::LineHeight};
use crate::prelude::*;

#[derive(Bundle, Debug, Clone)]
pub struct EasyTextStyle {
  pub text_shadow: TextShadow,
  pub text_font: TextFont,
  pub text_color: TextColor,
  pub text_layout: TextLayout,
  pub line_height: LineHeight,
}

impl Default for EasyTextStyle {
  fn default() -> Self {
    Self {
      text_shadow: TextShadow {
        color: BLACK.into(),
        offset: Vec2::ZERO,
      },
      text_font: TextFont::default(),
      text_color: TextColor::default(),
      text_layout: TextLayout::default(),
      line_height: LineHeight::default(),
    }
  }
}

pub trait EasyTextStyleExt: Sized {
  fn easy_text_style_mut(&mut self) -> &mut EasyTextStyle;

  fn with_color(mut self, text_color: Color) -> Self {
    self.easy_text_style_mut().text_color = TextColor(text_color);
    self
  }

  fn with_text_shadow(mut self, text_shadow: TextShadow) -> Self {
    self.easy_text_style_mut().text_shadow = text_shadow;
    self
  }

  fn with_shadow(mut self, text_shadow: TextShadow) -> Self {
    self.easy_text_style_mut().text_shadow = text_shadow;
    self
  }

  fn with_font_family(mut self, font: Handle<Font>) -> Self {
    self.easy_text_style_mut().text_font.font = font;
    self
  }

  fn with_font_size(mut self, font_size: f32) -> Self {
    self.easy_text_style_mut().text_font.font_size = font_size;
    self
  }

  fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
    self.easy_text_style_mut().text_font.weight = font_weight;
    self
  }

  fn with_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
    self.easy_text_style_mut().text_font.font_smoothing = font_smoothing;
    self
  }

  fn with_features(mut self, font_features: FontFeatures) -> Self {
    self.easy_text_style_mut().text_font.font_features = font_features;
    self
  }
  fn with_justify(mut self, justify: Justify) -> Self {
    self.easy_text_style_mut().text_layout.justify = justify;
    self
  }

  fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
    self.easy_text_style_mut().text_layout.linebreak = linebreak;
    self
  }

  fn with_line_height(mut self, line_height: LineHeight) -> Self {
    self.easy_text_style_mut().line_height = line_height;
    self
  }
}

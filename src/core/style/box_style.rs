use bevy::prelude::*;

#[derive(Bundle, Default, Debug, Clone)]
pub struct EasyBoxStyle {
  pub background_color: BackgroundColor,
  pub border_color: BorderColor,
  pub border_gradient: BorderGradient,
  pub background_gradient: BackgroundGradient,
  pub box_shadow: BoxShadow,
  pub outline: Outline,
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

  fn with_border_color_top(mut self, color: Color) -> Self {
    self.easy_style_mut().border_color.top = color;
    self
  }

  fn with_border_color_right(mut self, color: Color) -> Self {
    self.easy_style_mut().border_color.right = color;
    self
  }

  fn with_border_color_bottom(mut self, color: Color) -> Self {
    self.easy_style_mut().border_color.bottom = color;
    self
  }

  fn with_border_color_left(mut self, color: Color) -> Self {
    self.easy_style_mut().border_color.left = color;
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

  fn with_background_gradient(
    mut self,
    background_gradient: BackgroundGradient,
  ) -> Self {
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

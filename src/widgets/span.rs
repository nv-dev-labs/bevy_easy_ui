use bevy::prelude::*;

use crate::core::parts::{
  EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt,
  EasyTextProps, EasyTextPropsExt,
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan {
  pub text_span: TextSpan,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
  pub stack_style: EasyStackStyle,
}

#[derive(Default, Debug)]
pub struct EasySpanStyle {
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStackStyleExt for EasySpan {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.stack_style
  }
}

impl EasyBoxStyleExt for EasySpan {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.box_style
  }
}

impl EasyTextPropsExt for EasySpan {
  fn easy_props_mut(&mut self) -> &mut EasyTextProps {
    &mut self.text_style
  }
}

//>--------------------- BUILDER API ---------------------

impl EasySpan {
  pub fn new(text: &str) -> Self {
    EasySpan {
      text_span: TextSpan::new(text),
      box_style: EasyBoxStyle::default(),
      text_style: EasyTextProps::default(),
      stack_style: EasyStackStyle::default(),
    }
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.text_span = TextSpan::new(text);
    self
  }

  pub fn with_style(mut self, style: EasySpanStyle) -> Self {
    self.box_style = style.box_style;
    self.text_style = style.text_style;
    self.stack_style = style.stack_style;
    self
  }
}

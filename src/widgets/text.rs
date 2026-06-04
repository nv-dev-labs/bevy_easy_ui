use bevy::prelude::*;

use crate::core::{
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyTextProps, EasyTextPropsExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyText {
  pub text: Text,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
}

#[derive(Default, Debug)]
pub struct EasyTextStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyBoxStyleExt for EasyText {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.box_style
  }
}

impl EasyTextPropsExt for EasyText {
  fn easy_props_mut(&mut self) -> &mut EasyTextProps {
    &mut self.text_style
  }
}

impl EasyNode for EasyText {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyText {
  pub fn new(text: &str) -> Self {
    EasyText {
      text: Text::new(text),
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      text_style: EasyTextProps::default(),
    }
  }

  pub fn with_style(mut self, style: EasyTextStyle) -> Self {
    self.node = style.node;
    self.text_style = style.text_style;
    self.box_style = style.box_style;
    self
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.text = Text::new(text);
    self
  }
}

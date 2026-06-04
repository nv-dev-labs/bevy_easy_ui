use bevy::{
  ecs::bundle::Bundle,
  ui::{
    Node,
    widget::{Label, Text},
  },
};

use crate::core::{
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyTextProps, EasyTextPropsExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyLabel {
  pub text: Text,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
  pub label: Label,
}

#[derive(Default, Debug)]
pub struct EasyLabelStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextProps,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyBoxStyleExt for EasyLabel {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.box_style
  }
}

impl EasyTextPropsExt for EasyLabel {
  fn easy_props_mut(&mut self) -> &mut EasyTextProps {
    &mut self.text_style
  }
}

impl EasyNode for EasyLabel {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyLabel {
  pub fn new(text: &str) -> Self {
    EasyLabel {
      text: Text::new(text),
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      text_style: EasyTextProps::default(),
      label: Label,
    }
  }

  pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
    self.node = style.node;
    self.box_style = style.box_style;
    self.text_style = style.text_style;
    self
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.text = Text::new(text);
    self
  }
}

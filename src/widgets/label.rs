use bevy::{
  ecs::bundle::Bundle,
  ui::{
    Node,
    widget::{Label, Text},
  },
};

use crate::core::{
  node::EasyNode,
  parts::{
    EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt,
    EasyTextStyle, EasyTextStyleExt,
  },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyLabel {
  pub text: Text,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
  pub label: Label,
}

#[derive(Default, Debug)]
pub struct EasyLabelStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStackStyleExt for EasyLabel {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.stack_style
  }
}

impl EasyBoxStyleExt for EasyLabel {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.box_style
  }
}

impl EasyTextStyleExt for EasyLabel {
  fn easy_text_style_mut(&mut self) -> &mut EasyTextStyle {
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
      text_style: EasyTextStyle::default(),
      stack_style: EasyStackStyle::default(),
      label: Label,
    }
  }

  pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
    self.node = style.node;
    self.box_style = style.box_style;
    self.text_style = style.text_style;
    self.stack_style = style.stack_style;
    self
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.text = Text::new(text);
    self
  }
}

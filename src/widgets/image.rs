use bevy::prelude::*;

use crate::core::{
  image_node::EasyImageNode,
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyImage {
  pub image_node: ImageNode,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

#[derive(Default, Debug)]
pub struct EasyImageStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStackStyleExt for EasyImage {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.stack_style
  }
}

impl EasyBoxStyleExt for EasyImage {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.box_style
  }
}

impl EasyImageNode for EasyImage {
  fn node_mut(&mut self) -> &mut ImageNode {
    &mut self.image_node
  }
}

impl EasyNode for EasyImage {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyImage {
  pub fn new(image: Handle<Image>) -> EasyImage {
    EasyImage {
      image_node: ImageNode::new(image),
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }

  pub fn with_style(mut self, style: EasyImageStyle) -> Self {
    self.node = style.node;
    self.box_style = style.box_style;
    self.stack_style = style.stack_style;
    self
  }
}

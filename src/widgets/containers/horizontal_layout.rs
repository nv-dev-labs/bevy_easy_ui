use bevy::prelude::*;

use crate::core::{
  container::{Container, PushChild, PushObserver},
  element::EasyElement,
  node::EasyNode,
  style::{
    box_style::EasyBoxStyle, box_style::EasyBoxStyleExt,
    stack_style::EasyStackStyle, stack_style::EasyStackStyleExt,
  },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyHorizontalLayout {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyHorizontalLayoutContainer {
  bundle: EasyHorizontalLayout,
  children: Vec<EasyElement>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyHorizontalLayoutStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyStackStyleExt for EasyHorizontalLayoutContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyHorizontalLayoutContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl EasyBoxStyleExt for EasyHorizontalLayoutContainer {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl Container for EasyHorizontalLayoutContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyHorizontalLayout::default_bundle())
  }
  fn take_children(&mut self) -> Vec<EasyElement> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild for EasyHorizontalLayoutContainer {
  fn push_child(&mut self, c: EasyElement) {
    self.children.push(c);
  }
}
impl PushObserver for EasyHorizontalLayoutContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyHorizontalLayout {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyHorizontalLayoutContainer {
    EasyHorizontalLayoutContainer {
      bundle: EasyHorizontalLayout::default_bundle(),
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyHorizontalLayout {
      node: Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        ..default()
      },
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyHorizontalLayoutContainer {
  pub fn with_style(mut self, style: EasyHorizontalLayoutStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }
}

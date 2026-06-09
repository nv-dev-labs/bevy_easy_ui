use bevy::prelude::*;

use crate::core::{
  container::{Container, PushChild, PushObserver},
  element::EasyElement,
  node::EasyNode,
  parts::{box_style::EasyBoxStyle, box_style::EasyBoxStyleExt, stack_style::EasyStackStyle, stack_style::EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyVerticalLayout {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyVerticalLayoutContainer {
  bundle: EasyVerticalLayout,
  children: Vec<EasyElement>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyVerticalLayoutStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyBoxStyleExt for EasyVerticalLayoutContainer {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyStackStyleExt for EasyVerticalLayoutContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyVerticalLayoutContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl Container for EasyVerticalLayoutContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyVerticalLayout::default_bundle())
  }
  fn take_children(&mut self) -> Vec<EasyElement> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild for EasyVerticalLayoutContainer {
  fn push_child(&mut self, c: EasyElement) {
    self.children.push(c);
  }
}
impl PushObserver for EasyVerticalLayoutContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyVerticalLayout {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyVerticalLayoutContainer {
    EasyVerticalLayoutContainer {
      bundle: EasyVerticalLayout {
        node: Node {
          display: Display::Flex,
          flex_direction: FlexDirection::Column,
          ..default()
        },
        box_style: EasyBoxStyle::default(),
        stack_style: EasyStackStyle::default(),
      },
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyVerticalLayout {
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyVerticalLayoutContainer {
  pub fn with_style(mut self, style: EasyVerticalLayoutStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }
}

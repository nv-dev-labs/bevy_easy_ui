use bevy::prelude::*;

use crate::core::{
  container::{Container, PushChild, PushObserver},
  element::EasyElement,
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyHorizontalScrollLayout {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyHorizontalScrollLayoutContainer {
  bundle: EasyHorizontalScrollLayout,
  children: Vec<EasyElement>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyHorizontalScrollLayoutStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyStackStyleExt for EasyHorizontalScrollLayoutContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyHorizontalScrollLayoutContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl EasyBoxStyleExt for EasyHorizontalScrollLayoutContainer {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl Container for EasyHorizontalScrollLayoutContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(
      &mut self.bundle,
      EasyHorizontalScrollLayout::default_bundle(),
    )
  }
  fn take_children(&mut self) -> Vec<EasyElement> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild for EasyHorizontalScrollLayoutContainer {
  fn push_child(&mut self, c: EasyElement) {
    self.children.push(c);
  }
}
impl PushObserver for EasyHorizontalScrollLayoutContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- DEFAULT OBSERVERS ---------------------

//>--------------------- BUILDER API ---------------------

impl EasyHorizontalScrollLayout {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyHorizontalScrollLayoutContainer {
    EasyHorizontalScrollLayoutContainer {
      bundle: EasyHorizontalScrollLayout {
        node: Node {
          display: Display::Flex,
          flex_direction: FlexDirection::Row,
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
    EasyHorizontalScrollLayout {
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyHorizontalScrollLayoutContainer {
  pub fn with_style(mut self, style: EasyHorizontalScrollLayoutStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }
}

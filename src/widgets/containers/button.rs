use bevy::prelude::*;

use crate::core::{
  container::{Container, PushChild, PushObserver},
  element::EasyElement,
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyButton {
  pub button: Button,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyButtonContainer {
  bundle: EasyButton,
  children: Vec<EasyElement>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyButtonStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyBoxStyleExt for EasyButtonContainer {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyStackStyleExt for EasyButtonContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyButtonContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl Container for EasyButtonContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyButton::default_bundle())
  }
  fn take_children(&mut self) -> Vec<EasyElement> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild for EasyButtonContainer {
  fn push_child(&mut self, c: EasyElement) {
    self.children.push(c);
  }
}
impl PushObserver for EasyButtonContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyButton {
  pub fn new() -> EasyButtonContainer {
    EasyButtonContainer {
      bundle: EasyButton {
        button: Button,
        node: Node {
          display: Display::Flex,
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        box_style: EasyBoxStyle::default(),
        stack_style: EasyStackStyle::default(),
      },
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  fn default_bundle() -> Self {
    EasyButton {
      button: Button,
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyButtonContainer {
  pub fn with_style(mut self, style: EasyButtonStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }
}

use bevy::{ecs::observer::Observer, prelude::*, ui_widgets::RadioGroup};

use crate::core::{
  container::{Container, PushChild, PushObserver},
  element::EasyElement,
  node::EasyNode,
  style::{
    box_style::{EasyBoxStyle, EasyBoxStyleExt},
    stack_style::{EasyStackStyle, EasyStackStyleExt},
  },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyRadioGroup {
  pub button: RadioGroup,
  pub name: Name,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyRadioGroupContainer {
  bundle: EasyRadioGroup,
  children: Vec<EasyElement>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyRadioGroupStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyBoxStyleExt for EasyRadioGroupContainer {
  fn easy_box_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyStackStyleExt for EasyRadioGroupContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyRadioGroupContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl Container<EasyElement> for EasyRadioGroupContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(
      &mut self.bundle,
      EasyRadioGroup::default_bundle(String::new()),
    )
  }
  fn take_children(&mut self) -> Vec<EasyElement> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild for EasyRadioGroupContainer {
  fn push_child(&mut self, c: EasyElement) {
    self.children.push(c);
  }
}
impl PushObserver for EasyRadioGroupContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyRadioGroup {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(name: &str) -> EasyRadioGroupContainer {
    EasyRadioGroupContainer {
      bundle: EasyRadioGroup::default_bundle(name.to_string()),
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle(name: String) -> Self {
    EasyRadioGroup {
      button: RadioGroup,
      name: Name::new(name),
      node: Node {
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyRadioGroupContainer {
  pub fn with_style(mut self, style: EasyRadioGroupStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }

  pub fn with_name(mut self, name: String) -> Self {
    self.bundle.name = Name::new(name);
    self
  }
}

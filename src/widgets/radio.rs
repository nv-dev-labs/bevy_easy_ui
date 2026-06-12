use bevy::{
  ecs::system::IntoObserverSystem, picking::hover::Hovered, prelude::*,
  ui_widgets::RadioButton,
};

use crate::core::{
  container::WithObservers,
  node::EasyNode,
  style::{
    box_style::{EasyBoxStyle, EasyBoxStyleExt},
    stack_style::{EasyStackStyle, EasyStackStyleExt},
  },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyRadioButton {
  pub button: RadioButton,
  pub hovered: Hovered,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyRadioButtonBuilder {
  bundle: EasyRadioButton,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyRadioButtonStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyBoxStyleExt for EasyRadioButtonBuilder {
  fn easy_box_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyStackStyleExt for EasyRadioButtonBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyRadioButtonBuilder {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl WithObservers for EasyRadioButtonBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyRadioButton::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyRadioButton {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyRadioButtonBuilder {
    EasyRadioButtonBuilder {
      bundle: EasyRadioButton::default_bundle(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyRadioButton {
      button: RadioButton,
      hovered: Hovered::default(),
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

impl EasyRadioButtonBuilder {
  pub fn with_style(mut self, style: EasyRadioButtonStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }

  pub fn with_observer<E, ObsB, M>(
    mut self,
    observer: impl IntoObserverSystem<E, ObsB, M> + 'static,
  ) -> Self
  where
    E: Event,
    ObsB: Bundle,
  {
    self.observers.push(Observer::new(observer));
    self
  }
}

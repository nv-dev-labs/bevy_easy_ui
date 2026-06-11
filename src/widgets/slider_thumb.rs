use crate::prelude::*;

use bevy::{
  ecs::{
    bundle::Bundle, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  prelude::*,
};

use crate::core::{
  container::WithObservers, node::EasyNode, style::stack_style::EasyStackStyle,
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySliderThumb {
  pub slider_thumb: SliderThumb,
  pub node: Node,
  pub stack_style: EasyStackStyle,
  pub box_style: EasyBoxStyle,
}

pub struct EasySliderThumbBuilder {
  bundle: EasySliderThumb,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasySliderThumbStyle {
  pub node: Node,
  pub stack_style: EasyStackStyle,
  pub box_style: EasyBoxStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl WithObservers for EasySliderThumbBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasySliderThumb::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

impl EasyStackStyleExt for EasySliderThumbBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasySliderThumbBuilder {
  fn easy_box_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyNode for EasySliderThumbBuilder {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasySliderThumb {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasySliderThumbBuilder {
    EasySliderThumbBuilder {
      bundle: EasySliderThumb::default_bundle(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasySliderThumb {
      slider_thumb: SliderThumb,
      node: Node::default(),
      stack_style: EasyStackStyle::default(),
      box_style: EasyBoxStyle::default(),
    }
  }
}

impl EasySliderThumbBuilder {
  pub fn with_style(mut self, style: EasySliderThumbStyle) -> Self {
    self.bundle.stack_style = style.stack_style;
    self.bundle.box_style = style.box_style;
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

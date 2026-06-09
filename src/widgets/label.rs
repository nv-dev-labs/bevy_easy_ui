use bevy::{
  ecs::{
    bundle::Bundle, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  ui::{
    Node,
    widget::{Label, Text},
  },
};

use crate::core::{
  container::WithObservers,
  node::EasyNode,
  parts::{
    box_style::EasyBoxStyle, box_style::EasyBoxStyleExt, stack_style::EasyStackStyle, stack_style::EasyStackStyleExt,
    text_style::EasyTextStyle, text_style::EasyTextStyleExt,
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

pub struct EasyLabelBuilder {
  bundle: EasyLabel,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyLabelStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl WithObservers for EasyLabelBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyLabel::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

impl EasyStackStyleExt for EasyLabelBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasyLabelBuilder {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyTextStyleExt for EasyLabelBuilder {
  fn easy_text_style_mut(&mut self) -> &mut EasyTextStyle {
    &mut self.bundle.text_style
  }
}

impl EasyNode for EasyLabelBuilder {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyLabel {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(text: &str) -> EasyLabelBuilder {
    EasyLabelBuilder {
      bundle: EasyLabel {
        text: Text::new(text),
        node: Node::default(),
        box_style: EasyBoxStyle::default(),
        text_style: EasyTextStyle::default(),
        stack_style: EasyStackStyle::default(),
        label: Label,
      },
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyLabel {
      text: Text::default(),
      node: Node::default(),
      box_style: EasyBoxStyle::default(),
      text_style: EasyTextStyle::default(),
      stack_style: EasyStackStyle::default(),
      label: Label,
    }
  }
}

impl EasyLabelBuilder {
  pub fn with_style(mut self, style: EasyLabelStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.text_style = style.text_style;
    self.bundle.stack_style = style.stack_style;
    self
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.bundle.text = Text::new(text);
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

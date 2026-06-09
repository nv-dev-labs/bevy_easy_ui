use bevy::{
  ecs::{
    bundle::Bundle, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  prelude::*,
};

use crate::core::{
  container::WithObservers,
  parts::{
    box_style::EasyBoxStyle, box_style::EasyBoxStyleExt, stack_style::EasyStackStyle, stack_style::EasyStackStyleExt,
    text_style::EasyTextStyle, text_style::EasyTextStyleExt,
  },
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySpan {
  pub text_span: TextSpan,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasySpanBuilder {
  bundle: EasySpan,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasySpanStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl WithObservers for EasySpanBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasySpan::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

impl EasyStackStyleExt for EasySpanBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasySpanBuilder {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyTextStyleExt for EasySpanBuilder {
  fn easy_text_style_mut(&mut self) -> &mut EasyTextStyle {
    &mut self.bundle.text_style
  }
}

//>--------------------- BUILDER API ---------------------

impl EasySpan {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(text: &str) -> EasySpanBuilder {
    EasySpanBuilder {
      bundle: EasySpan {
        text_span: TextSpan::new(text),
        box_style: EasyBoxStyle::default(),
        text_style: EasyTextStyle::default(),
        stack_style: EasyStackStyle::default(),
      },
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasySpan {
      text_span: TextSpan::new(""),
      box_style: EasyBoxStyle::default(),
      text_style: EasyTextStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasySpanBuilder {
  pub fn with_text(mut self, text: &str) -> Self {
    self.bundle.text_span = TextSpan::new(text);
    self
  }

  pub fn with_style(mut self, style: EasySpanStyle) -> Self {
    self.bundle.box_style = style.box_style;
    self.bundle.text_style = style.text_style;
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

use crate::prelude::*;
use bevy::{
  ecs::{
    bundle::Bundle, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  prelude::*,
  text::{FontFeatures, FontSmoothing, LineHeight},
};
use bevy_ui_text_input::{
  TextInputBuffer, TextInputMode, TextInputNode, TextInputQueue, TextInputStyle,
};

use crate::core::{
  container::WithObservers,
  node::EasyNode,
  style::{stack_style::EasyStackStyle, stack_style::EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyTextInput {
  pub node: Node,
  pub stack_style: EasyStackStyle,
  pub text_input: TextInputNode,
  pub text_buffer: TextInputBuffer,
  pub text_font: TextFont,
  pub text_style_input: TextInputStyle,
  pub text_color: TextColor,
  pub text_queue: TextInputQueue,
  pub line_height: LineHeight,
}

pub struct EasyTextInputBuilder {
  bundle: EasyTextInput,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyTextInputStyle {
  pub node: Node,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl WithObservers for EasyTextInputBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyTextInput::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

impl EasyStackStyleExt for EasyTextInputBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyNode for EasyTextInputBuilder {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyTextInput {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyTextInputBuilder {
    EasyTextInputBuilder {
      bundle: EasyTextInput::default_bundle(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyTextInput {
      text_input: TextInputNode::default(),
      text_buffer: TextInputBuffer::default(),
      text_font: TextFont::default(),
      text_style_input: TextInputStyle::default(),
      text_color: BLACK.into(),
      text_queue: TextInputQueue::default(),
      line_height: LineHeight::default(),
      node: Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.),
        height: Val::Px(40.),
        ..Default::default()
      },
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyTextInputBuilder {
  pub fn with_style(mut self, style: EasyTextInputStyle) -> Self {
    self.bundle.stack_style = style.stack_style;
    self
  }

  // TODO: Add setters for all text input style properties (TextInputStyle, TextFont, TextColor, LineHeight)

  pub fn with_single_line(mut self, single_line: bool) -> Self {
    self.bundle.text_input.mode = if single_line {
      TextInputMode::SingleLine
    } else {
      TextInputMode::default()
    };
    self
  }

  pub fn with_color(mut self, color: Color) -> Self {
    self.bundle.text_color = color.into();
    self
  }

  pub fn with_font(mut self, font: Handle<Font>) -> Self {
    self.bundle.text_font.font = font;
    self
  }

  pub fn with_font_size(mut self, font_size: f32) -> Self {
    self.bundle.text_font.font_size = font_size;
    self
  }

  pub fn with_smoothing(mut self, smoothing: FontSmoothing) -> Self {
    self.bundle.text_font.font_smoothing = smoothing;
    self
  }

  pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
    self.bundle.text_font.weight = font_weight;
    self
  }

  pub fn with_font_features(mut self, font_features: FontFeatures) -> Self {
    self.bundle.text_font.font_features = font_features;
    self
  }

  pub fn with_line_height(mut self, line_height: LineHeight) -> Self {
    self.bundle.line_height = line_height;
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

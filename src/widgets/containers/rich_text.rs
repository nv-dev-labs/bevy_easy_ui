use bevy::prelude::*;

use crate::{
  core::{
    container::{Container, PushChild, PushObserver},
    node::EasyNode,
    parts::{
      box_style::EasyBoxStyle, box_style::EasyBoxStyleExt,
      stack_style::EasyStackStyle, stack_style::EasyStackStyleExt,
      text_style::EasyTextStyle, text_style::EasyTextStyleExt,
    },
  },
  widgets::span::EasySpanBuilder,
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyRichText {
  pub text: Text,
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyRichTextContainer {
  bundle: EasyRichText,
  children: Vec<EasySpanBuilder>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyRichTextStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub text_style: EasyTextStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl EasyStackStyleExt for EasyRichTextContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasyRichTextContainer {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyTextStyleExt for EasyRichTextContainer {
  fn easy_text_style_mut(&mut self) -> &mut EasyTextStyle {
    &mut self.bundle.text_style
  }
}

impl EasyNode for EasyRichTextContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

impl Container<EasySpanBuilder> for EasyRichTextContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyRichText::default_bundle())
  }
  fn take_children(&mut self) -> Vec<EasySpanBuilder> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild<EasySpanBuilder> for EasyRichTextContainer {
  fn push_child(&mut self, c: EasySpanBuilder) {
    self.children.push(c);
  }
}
impl PushObserver<EasySpanBuilder> for EasyRichTextContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyRichText {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasyRichTextContainer {
    EasyRichTextContainer {
      bundle: EasyRichText {
        text: Text::new(""),
        node: Node::default(),
        text_style: EasyTextStyle::default(),
        box_style: EasyBoxStyle::default(),
        stack_style: EasyStackStyle::default(),
      },
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasyRichText {
      text: Text::new(""),
      node: Node::default(),
      text_style: EasyTextStyle::default(),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyRichTextContainer {
  pub fn with_style(mut self, style: EasyRichTextStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.text_style = style.text_style;
    self.bundle.stack_style = style.stack_style;
    self
  }
}

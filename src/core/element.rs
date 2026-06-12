use bevy::prelude::*;

use crate::{
  core::container::{Container, WithObservers},
  widgets::{
    checkbox::EasyCheckboxBuilder,
    containers::{
      button::EasyButtonContainer,
      horizontal_layout::EasyHorizontalLayoutContainer,
      radio_group::EasyRadioGroupContainer, rich_text::EasyRichTextContainer,
      slider::EasySliderContainer,
      vertical_layout::EasyVerticalLayoutContainer,
    },
    image::EasyImageBuilder,
    label::EasyLabelBuilder,
    radio::EasyRadioButtonBuilder,
    slider_thumb::EasySliderThumbBuilder,
    span::EasySpanBuilder,
    text::EasyTextBuilder,
    text_input::EasyTextInputBuilder,
  },
};

//>--------------------- ALL ELEMENTS ---------------------

pub enum EasyElement {
  // Containers (i.e. elements that can have children):
  ButtonContainer(EasyButtonContainer),
  RichTextContainer(EasyRichTextContainer),
  VerticalContainer(EasyVerticalLayoutContainer),
  HorizontalContainer(EasyHorizontalLayoutContainer),
  Slider(EasySliderContainer),
  RadioGroup(EasyRadioGroupContainer),

  // Non-containers (i.e. leaf nodes):
  Image(EasyImageBuilder),
  Text(EasyTextBuilder),
  Label(EasyLabelBuilder),
  Span(EasySpanBuilder),
  TextInput(EasyTextInputBuilder),
  Checkbox(EasyCheckboxBuilder),
  SliderThumb(EasySliderThumbBuilder),
  RadioButton(EasyRadioButtonBuilder),
}

//>--------------------- IMPLEMENTATIONS ---------------------

//> Impl for containers
impl From<EasyButtonContainer> for EasyElement {
  fn from(b: EasyButtonContainer) -> Self {
    EasyElement::ButtonContainer(b)
  }
}
impl From<EasyRichTextContainer> for EasyElement {
  fn from(t: EasyRichTextContainer) -> Self {
    EasyElement::RichTextContainer(t)
  }
}
impl From<EasyVerticalLayoutContainer> for EasyElement {
  fn from(c: EasyVerticalLayoutContainer) -> Self {
    EasyElement::VerticalContainer(c)
  }
}
impl From<EasyHorizontalLayoutContainer> for EasyElement {
  fn from(c: EasyHorizontalLayoutContainer) -> Self {
    EasyElement::HorizontalContainer(c)
  }
}
impl From<EasySliderContainer> for EasyElement {
  fn from(s: EasySliderContainer) -> Self {
    EasyElement::Slider(s)
  }
}
impl From<EasyRadioGroupContainer> for EasyElement {
  fn from(r: EasyRadioGroupContainer) -> Self {
    EasyElement::RadioGroup(r)
  }
}

//> Impl for non-containers
impl From<EasyImageBuilder> for EasyElement {
  fn from(i: EasyImageBuilder) -> Self {
    EasyElement::Image(i)
  }
}
impl From<EasyTextBuilder> for EasyElement {
  fn from(t: EasyTextBuilder) -> Self {
    EasyElement::Text(t)
  }
}
impl From<EasyLabelBuilder> for EasyElement {
  fn from(l: EasyLabelBuilder) -> Self {
    EasyElement::Label(l)
  }
}
impl From<EasySpanBuilder> for EasyElement {
  fn from(s: EasySpanBuilder) -> Self {
    EasyElement::Span(s)
  }
}
impl From<EasyTextInputBuilder> for EasyElement {
  fn from(t: EasyTextInputBuilder) -> Self {
    EasyElement::TextInput(t)
  }
}
impl From<EasyCheckboxBuilder> for EasyElement {
  fn from(c: EasyCheckboxBuilder) -> Self {
    EasyElement::Checkbox(c)
  }
}
impl From<EasySliderThumbBuilder> for EasyElement {
  fn from(s: EasySliderThumbBuilder) -> Self {
    EasyElement::SliderThumb(s)
  }
}
impl From<EasyRadioButtonBuilder> for EasyElement {
  fn from(r: EasyRadioButtonBuilder) -> Self {
    EasyElement::RadioButton(r)
  }
}

impl EasyElement {
  /// Spawns this EasyElement in the world, as a child of the given parent.
  /// This is done by matching on the type of the element (container vs non-container)
  /// and calling the appropriate helper function to spawn it.
  pub fn spawn_in(self, p: &mut ChildSpawnerCommands) {
    match self {
      // Containers
      EasyElement::ButtonContainer(c) => spawn_container(c, p),
      EasyElement::RichTextContainer(c) => spawn_rich_text(c, p),
      EasyElement::VerticalContainer(c) => spawn_container(c, p),
      EasyElement::HorizontalContainer(c) => spawn_container(c, p),
      EasyElement::Slider(s) => spawn_slider(s, p),
      EasyElement::RadioGroup(r) => spawn_container(r, p),

      // Non-containers
      EasyElement::Image(i) => spawn(i, p),
      EasyElement::Text(t) => spawn(t, p),
      EasyElement::Label(l) => spawn(l, p),
      EasyElement::Span(s) => spawn(s, p),
      EasyElement::TextInput(t) => spawn(t, p),
      EasyElement::Checkbox(c) => spawn(c, p),
      EasyElement::SliderThumb(s) => spawn(s, p),
      EasyElement::RadioButton(r) => spawn(r, p),
    }
  }
}

/// **Helper function** to spawn an EasyElement that is a non-container (i.e. can't have children).
/// It spawns the element itself and then spawns its observers.
fn spawn(mut e: impl WithObservers, p: &mut ChildSpawnerCommands) {
  let entity = p.spawn(e.take_bundle()).id();
  for observer in e.take_observers() {
    p.commands().spawn(observer.with_entity(entity));
  }
}

/// **Helper function** to spawn an EasyElement that is a container (i.e. can have children).
/// It spawns the container itself, then recursively spawns its children, and finally spawns its observers.
fn spawn_container(
  mut c: impl Container<EasyElement>,
  p: &mut ChildSpawnerCommands,
) {
  let entity = p.spawn(c.take_bundle()).id();
  let kids = c.take_children();
  p.commands().entity(entity).with_children(|sub| {
    for child in kids {
      child.spawn_in(sub);
    }
  });
  for observer in c.take_observers() {
    p.commands().spawn(observer.with_entity(entity));
  }
}

/// **Helper function** to spawn an EasyRichTextContainer, which is a special case because it has a different bundle and children type
/// (EasySpanBuilder only instead of generic EasyElement).
fn spawn_rich_text(mut t: EasyRichTextContainer, p: &mut ChildSpawnerCommands) {
  let entity = p.spawn(t.take_bundle()).id();
  let kids = t.take_children();
  for observer in t.take_observers() {
    p.commands().spawn(observer.with_entity(entity));
  }
  p.commands().entity(entity).with_children(|sub| {
    for child in kids {
      spawn(child, sub);
    }
  });
}

/// **Helper function** to spawn an EasySliderContainer, which is a special case because it has a different bundle and children type
/// (EasySliderThumbBuilder only instead of generic EasyElement).
fn spawn_slider(mut s: EasySliderContainer, p: &mut ChildSpawnerCommands) {
  let entity = p.spawn(s.take_bundle()).id();
  let kids = s.take_children();
  for observer in s.take_observers() {
    p.commands().spawn(observer.with_entity(entity));
  }
  p.commands().entity(entity).with_children(|sub| {
    for child in kids {
      spawn(child, sub);
    }
  });
}

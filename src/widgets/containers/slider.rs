use crate::{
  core::container::{PushChild, PushObserver},
  prelude::*,
  widgets::slider_thumb::EasySliderThumbBuilder,
};

use bevy::{
  ecs::{
    bundle::Bundle, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  prelude::*,
};

use crate::core::{node::EasyNode, style::stack_style::EasyStackStyle};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasySlider {
  pub slider: Slider,
  pub slider_step: SliderStep,
  pub slider_precision: SliderPrecision,
  pub slider_value: SliderValue,
  pub slider_range: SliderRange,
  pub node: Node,
  pub stack_style: EasyStackStyle,
  pub box_style: EasyBoxStyle,
}

pub struct EasySliderContainer {
  bundle: EasySlider,
  children: Vec<EasySliderThumbBuilder>,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasySliderStyle {
  pub node: Node,
  pub stack_style: EasyStackStyle,
  pub box_style: EasyBoxStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl Container<EasySliderThumbBuilder> for EasySliderContainer {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasySlider::default_bundle())
  }
  fn take_children(&mut self) -> Vec<EasySliderThumbBuilder> {
    std::mem::take(&mut self.children)
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}
impl PushChild<EasySliderThumbBuilder> for EasySliderContainer {
  fn push_child(&mut self, c: EasySliderThumbBuilder) {
    self.children.push(c);
  }
}
impl PushObserver<EasySliderThumbBuilder> for EasySliderContainer {
  fn push_observer(&mut self, o: Observer) {
    self.observers.push(o);
  }
}

impl EasyStackStyleExt for EasySliderContainer {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasySliderContainer {
  fn easy_box_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyNode for EasySliderContainer {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

//>--------------------- Container API ---------------------

impl EasySlider {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> EasySliderContainer {
    EasySliderContainer {
      bundle: EasySlider::default_bundle(),
      children: Vec::new(),
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    EasySlider {
      slider: Slider::default(),
      slider_step: SliderStep::default(),
      slider_precision: SliderPrecision::default(),
      slider_value: SliderValue::default(),
      slider_range: SliderRange::default(),
      node: Node::default(),
      stack_style: EasyStackStyle::default(),
      box_style: EasyBoxStyle::default(),
    }
  }
}

impl EasySliderContainer {
  pub fn with_style(mut self, style: EasySliderStyle) -> Self {
    self.bundle.stack_style = style.stack_style;
    self.bundle.box_style = style.box_style;
    self
  }

  pub fn with_track_click(mut self, track_click: TrackClick) -> Self {
    self.bundle.slider.track_click = track_click;
    self
  }

  pub fn with_range(mut self, min: f32, max: f32) -> Self {
    self.bundle.slider_range = SliderRange::new(min, max);
    self
  }

  pub fn with_value(mut self, value: f32) -> Self {
    self.bundle.slider_value = SliderValue(value);
    self
  }

  pub fn with_step(mut self, step: f32) -> Self {
    self.bundle.slider_step = SliderStep(step);
    self
  }

  pub fn with_precision(mut self, precision: i32) -> Self {
    self.bundle.slider_precision = SliderPrecision(precision);
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

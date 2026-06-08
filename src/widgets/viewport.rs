use bevy::{
  ecs::{
    bundle::Bundle, entity::Entity, event::Event, observer::Observer,
    system::IntoObserverSystem,
  },
  ui::{Node, widget::ViewportNode},
};

use crate::core::{
  container::WithObservers,
  node::EasyNode,
  parts::{EasyBoxStyle, EasyBoxStyleExt, EasyStackStyle, EasyStackStyleExt},
};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyViewport {
  pub node: Node,
  pub viewport: ViewportNode,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

pub struct EasyViewportBuilder {
  bundle: EasyViewport,
  observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyViewportStyle {
  pub node: Node,
  pub box_style: EasyBoxStyle,
  pub stack_style: EasyStackStyle,
}

//>--------------------- ACCESSOR IMPLS ---------------------

impl WithObservers for EasyViewportBuilder {
  fn take_bundle(&mut self) -> impl Bundle {
    std::mem::replace(&mut self.bundle, EasyViewport::default_bundle())
  }
  fn take_observers(&mut self) -> Vec<Observer> {
    std::mem::take(&mut self.observers)
  }
}

impl EasyStackStyleExt for EasyViewportBuilder {
  fn easy_stack_style_mut(&mut self) -> &mut EasyStackStyle {
    &mut self.bundle.stack_style
  }
}

impl EasyBoxStyleExt for EasyViewportBuilder {
  fn easy_style_mut(&mut self) -> &mut EasyBoxStyle {
    &mut self.bundle.box_style
  }
}

impl EasyNode for EasyViewportBuilder {
  fn node_mut(&mut self) -> &mut Node {
    &mut self.bundle.node
  }
}

//>--------------------- BUILDER API ---------------------

impl EasyViewport {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(camera: Entity) -> EasyViewportBuilder {
    EasyViewportBuilder {
      bundle: EasyViewport {
        node: Node::default(),
        viewport: ViewportNode::new(camera),
        box_style: EasyBoxStyle::default(),
        stack_style: EasyStackStyle::default(),
      },
      observers: Vec::new(),
    }
  }

  pub fn default_bundle() -> Self {
    // A "default" ViewportNode has no camera — the caller is expected to
    // overwrite it via `with_target_camera` or `new(camera)` before spawning.
    // We use the world entity placeholder (Entity::PLACEHOLDER) since
    // `ViewportNode::new` requires one.
    EasyViewport {
      node: Node::default(),
      viewport: ViewportNode::new(Entity::PLACEHOLDER),
      box_style: EasyBoxStyle::default(),
      stack_style: EasyStackStyle::default(),
    }
  }
}

impl EasyViewportBuilder {
  pub fn with_style(mut self, style: EasyViewportStyle) -> Self {
    self.bundle.node = style.node;
    self.bundle.box_style = style.box_style;
    self.bundle.stack_style = style.stack_style;
    self
  }

  pub fn with_target_camera(mut self, camera: Entity) -> Self {
    self.bundle.viewport = ViewportNode::new(camera);
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

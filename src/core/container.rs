use bevy::ecs::observer::Observer;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use crate::core::element::EasyElement;

//>--------------------- CONTAINERS (observers & children) ---------------------

/// A container is an element that has both children and observers.
pub trait Container<C: Into<EasyElement> = EasyElement>: Sized {
  /// Access to the final Bevy bundle to spawn (Button+Node+..., or just Node, etc.)
  fn take_bundle(&mut self) -> impl Bundle;
  /// Takes the Vec of children (to empty it on spawn)
  fn take_children(&mut self) -> Vec<C>;
  /// Takes the Vec of observers
  fn take_observers(&mut self) -> Vec<Observer>;

  /// Adds a child
  fn with_child(mut self, child: impl Into<C>) -> Self
  where
    Self: PushChild<C>,
  {
    self.push_child(child.into());
    self
  }

  /// Adds an observer
  fn with_observer<E, ObsB, M>(
    mut self,
    observer: impl IntoObserverSystem<E, ObsB, M> + 'static,
  ) -> Self
  where
    Self: PushObserver<C>,
    E: Event,
    ObsB: Bundle,
  {
    self.push_observer(Observer::new(observer));
    self
  }

  /// Spawns into the hierarchy
  fn spawn(self, commands: &mut Commands) -> Entity
  where
    Self: PushChild<C> + PushObserver<C>,
  {
    spawn_container(self, commands)
  }
}

/// A container that can have children pushed onto it.
pub trait PushChild<C: Into<EasyElement> = EasyElement>: Container<C> {
  fn push_child(&mut self, child: C);
}

/// A container that can have observers pushed onto it.
pub trait PushObserver<C: Into<EasyElement> = EasyElement>:
  Container<C>
{
  fn push_observer(&mut self, observer: Observer);
}

/// Spawns a container element, its children, and its observers.
fn spawn_container<C: Into<EasyElement>>(
  mut c: impl PushChild<C> + PushObserver<C>,
  commands: &mut Commands,
) -> Entity {
  let bundle = c.take_bundle();
  let children = c.take_children();
  let observers = c.take_observers();

  let entity = commands.spawn(bundle).id();
  commands.entity(entity).with_children(|p| {
    for child in children {
      let el: EasyElement = child.into();
      el.spawn_in(p);
    }
  });
  for observer in observers {
    commands.spawn(observer.with_entity(entity));
  }
  entity
}

//>--------------------- NON-CONTAINER ELEMENTS (observers only) ---------------------

/// A non-container element that has observers but no children.
/// For example, a Text is not a container because it can't have children, but it can be hovered.
pub trait WithObservers<C: Into<EasyElement> = EasyElement>: Sized {
  fn take_bundle(&mut self) -> impl Bundle;
  fn take_observers(&mut self) -> Vec<Observer>;
  /// Spawns a non-container element into the world
  fn spawn(self, commands: &mut Commands) -> Entity {
    spawn(self, commands)
  }
}

/// Spawns a non-container element and its observers.
fn spawn<C: Into<EasyElement>>(
  mut c: impl WithObservers<C>,
  commands: &mut Commands,
) -> Entity {
  let bundle = c.take_bundle();
  let observers = c.take_observers();

  let entity = commands.spawn(bundle).id();
  for observer in observers {
    commands.spawn(observer.with_entity(entity));
  }
  entity
}

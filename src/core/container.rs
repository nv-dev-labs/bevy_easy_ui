use bevy::ecs::observer::Observer;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use crate::core::element::EasyElement;

/// The heart of the system: a Container is just "something that can have
/// children, observers, and be spawned". We extend this trait to anything
/// that can live in the UI tree (leaf nodes like EasyText are excluded).
pub trait Container: Sized {
    /// Access to the final Bevy bundle to spawn (Button+Node+..., or just Node, etc.)
    fn take_bundle(&mut self) -> impl Bundle;
    /// Takes the Vec of children (to empty it on spawn)
    fn take_children(&mut self) -> Vec<EasyElement>;
    /// Takes the Vec of observers
    fn take_observers(&mut self) -> Vec<Observer>;

    /// Adds a child
    fn with_child(mut self, child: impl Into<EasyElement>) -> Self
    where
        Self: PushChild,
    {
        self.push_child(child.into());
        self
    }

    /// Adds an observer
    fn with_observer<E, ObsB, M>(mut self, observer: impl IntoObserverSystem<E, ObsB, M> + 'static) -> Self
    where
        Self: PushObserver,
        E: Event,
        ObsB: Bundle,
    {
        self.push_observer(Observer::new(observer));
        self
    }

    /// Spawns into the hierarchy
    fn spawn(self, commands: &mut Commands) -> Entity
    where
        Self: PushChild + PushObserver,
    {
        // internal "mut" version — see below
        spawn_container(self, commands)
    }
}

// Helpers so we can push into self from within the trait (by default, we
// store things in internal fields — each concrete type defines push_child/push_observer).
pub trait PushChild: Container {
    fn push_child(&mut self, child: EasyElement);
}
pub trait PushObserver: Container {
    fn push_observer(&mut self, observer: Observer);
}

fn spawn_container(mut c: impl Container + PushChild + PushObserver, commands: &mut Commands) -> Entity {
    let bundle = c.take_bundle();
    let children = c.take_children();
    let observers = c.take_observers();

    let entity = commands.spawn(bundle).id();
    commands.entity(entity).with_children(|p| {
        for child in children {
            child.spawn_in(p);
        }
    });
    for observer in observers {
        commands.spawn(observer.with_entity(entity));
    }
    entity
}

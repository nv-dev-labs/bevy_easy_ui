use bevy::ecs::observer::Observer;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use crate::core::element::EasyElement;

/// Le coeur du système : un Container, c'est juste "un truc qui peut avoir
/// des enfants, des observers, et être spawné". On étend ce trait à tout ce
/// qui peut se trouver dans l'arbre UI (les feuilles commme EasyText sont exclues).
pub trait Container: Sized {
    /// Accès au bundle Bevy final à spawn (Button+Node+..., ou juste Node, etc.)
    fn take_bundle(&mut self) -> impl Bundle;
    /// Récupère le Vec d'enfants (pour le vider lors du spawn)
    fn take_children(&mut self) -> Vec<EasyElement>;
    /// Récupère le Vec d'observers
    fn take_observers(&mut self) -> Vec<Observer>;

    /// Ajoute un enfant
    fn child(mut self, child: impl Into<EasyElement>) -> Self
    where
        Self: PushChild,
    {
        self.push_child(child.into());
        self
    }

    /// Ajoute un observer
    fn observe<E, ObsB, M>(mut self, observer: impl IntoObserverSystem<E, ObsB, M> + 'static) -> Self
    where
        Self: PushObserver,
        E: Event,
        ObsB: Bundle,
    {
        self.push_observer(Observer::new(observer));
        self
    }

    /// Spawn dans la hiérarchie
    fn spawn(self, commands: &mut Commands) -> Entity
    where
        Self: PushChild + PushObserver,
    {
        // version "mut" interne — voir plus bas
        spawn_container(self, commands)
    }
}

// Helpers pour pouvoir push dans self depuis le trait (par défaut, on stocke
// dans des champs internes — chaque type concret définit push_child/push_observer).
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

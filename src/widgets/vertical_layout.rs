use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

#[derive(Bundle)]
pub struct EasyVerticalLayout (
    Node,
);

pub struct EasyVerticalLayoutContainer {
    bundle: EasyVerticalLayout,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default)]
pub struct EasyVerticalLayoutStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>------------------------------------------

//? Implémentation de la partie "API de construction" d'EasyVerticalLayout, qui nous permet de construire la définition déclarative du layout (sans le spawn)
impl EasyVerticalLayout {
    /// Crée un builder qui EST DÉJÀ un container : on peut chaîner
    /// `.child(...)`, `.observe(...)`, `.spawn(commands)` directement.
    pub fn new() -> EasyVerticalLayoutContainer {
        EasyVerticalLayoutContainer {
            bundle: EasyVerticalLayout(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            }),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyVerticalLayout(Node::default())
    }

    pub fn with_style(mut self, style: EasyVerticalLayoutStyle) -> Self {
        self.0 = style.node;
        self
    }
}

impl EasyNode for EasyVerticalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.0
    }
}

//? Implémentation du trait Container pour EasyVerticalLayoutContainer, qui nous permet de le traiter comme un élément de l'arbre UI avec des enfants et des observers
impl Container for EasyVerticalLayoutContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyVerticalLayout::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasyElement> { std::mem::take(&mut self.children) }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild for EasyVerticalLayoutContainer {
    fn push_child(&mut self, c: EasyElement) { self.children.push(c); }
}
impl PushObserver for EasyVerticalLayoutContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>------------------------------------------

impl std::ops::Deref for EasyVerticalLayoutStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyVerticalLayout> for (Node,) {
    fn from(layout: EasyVerticalLayout) -> (Node,) {
       (layout.0,)
    }
}

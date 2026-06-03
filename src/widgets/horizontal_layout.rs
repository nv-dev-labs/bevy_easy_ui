use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

#[derive(Bundle)]
pub struct EasyHorizontalLayout (
    Node,
);

pub struct EasyHorizontalLayoutContainer {
    bundle: EasyHorizontalLayout,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default)]
pub struct EasyHorizontalLayoutStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>------------------------------------------

//? Implémentation de la partie "API de construction" d'EasyHorizontalLayout, qui nous permet de construire la définition déclarative du layout (sans le spawn)
impl EasyHorizontalLayout {
    /// Crée un builder qui EST DÉJÀ un container : on peut chaîner
    /// `.child(...)`, `.observe(...)`, `.spawn(commands)` directement.
    pub fn new() -> EasyHorizontalLayoutContainer {
        EasyHorizontalLayoutContainer {
            bundle: EasyHorizontalLayout(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyHorizontalLayout(Node::default())
    }

    pub fn with_style(mut self, style: EasyHorizontalLayoutStyle) -> Self {
        self.0 = style.node;
        self
    }
}

impl EasyNode for EasyHorizontalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.0
    }
}

//? Implémentation du trait Container pour EasyHorizontalLayoutContainer, qui nous permet de le traiter comme un élément de l'arbre UI avec des enfants et des observers
impl Container for EasyHorizontalLayoutContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyHorizontalLayout::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasyElement> { std::mem::take(&mut self.children) }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild for EasyHorizontalLayoutContainer {
    fn push_child(&mut self, c: EasyElement) { self.children.push(c); }
}
impl PushObserver for EasyHorizontalLayoutContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>------------------------------------------

impl std::ops::Deref for EasyHorizontalLayoutStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyHorizontalLayout> for (Node,) {
    fn from(layout: EasyHorizontalLayout) -> (Node,) {
       (layout.0,)
    }
}

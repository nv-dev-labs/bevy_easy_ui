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

//? Implementation of the "builder API" part of EasyVerticalLayout, which lets us build the declarative definition of the layout (without spawning it)
impl EasyVerticalLayout {
    /// Creates a builder that IS ALREADY a container: you can chain
    /// `.child(...)`, `.observe(...)`, `.spawn(commands)` directly.
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
}

impl EasyNode for EasyVerticalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.0
    }
}

impl EasyVerticalLayoutContainer {
    pub fn with_style(mut self, style: EasyVerticalLayoutStyle) -> Self {
        self.bundle.0 = style.node;
        self
    }
}

//? Implementation of the Container trait for EasyVerticalLayoutContainer, which lets us treat it as a node of the UI tree with children and observers
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

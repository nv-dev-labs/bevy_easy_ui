use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyVerticalLayout {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

pub struct EasyVerticalLayoutContainer {
    bundle: EasyVerticalLayout,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyVerticalLayoutStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyVerticalLayout {
    pub fn new() -> EasyVerticalLayoutContainer {
        EasyVerticalLayoutContainer {
            bundle: EasyVerticalLayout {
                node: Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor::default(),
                border_color: BorderColor::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyVerticalLayout {
            node: Node::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
        }
    }
}

impl EasyNode for EasyVerticalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
    }
}

impl EasyVerticalLayoutContainer {
    pub fn with_style(mut self, style: EasyVerticalLayoutStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.background_color = style.background_color;
        self.bundle.border_color = style.border_color;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.bundle.background_color = BackgroundColor(background_color);
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.bundle.border_color = BorderColor::all(border_color);
        self
    }
}

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

//>--------------------- HELPERS --------------------------

impl std::ops::Deref for EasyVerticalLayoutStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyVerticalLayout> for (Node, BackgroundColor, BorderColor) {
    fn from(layout: EasyVerticalLayout) -> (Node, BackgroundColor, BorderColor) {
       (layout.node, layout.background_color, layout.border_color)
    }
}

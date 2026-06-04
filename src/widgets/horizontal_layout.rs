use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyHorizontalLayout (
    Node,
    BackgroundColor,
    BorderColor,
);

pub struct EasyHorizontalLayoutContainer {
    bundle: EasyHorizontalLayout,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyHorizontalLayoutStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>--------------------- IMPLEMENTATION ---------------------

impl EasyHorizontalLayout {
    pub fn new() -> EasyHorizontalLayoutContainer {
        EasyHorizontalLayoutContainer {
            bundle: EasyHorizontalLayout(
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                BackgroundColor::default(), 
                BorderColor::default()
            ),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyHorizontalLayout(Node::default(), BackgroundColor::default(), BorderColor::default())
    }
}

impl EasyHorizontalLayoutContainer {
    pub fn with_style(mut self, style: EasyHorizontalLayoutStyle) -> Self {
        self.bundle.0 = style.node;
        self.bundle.1 = style.background_color;
        self.bundle.2 = style.border_color;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.bundle.1 = BackgroundColor(background_color);
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.bundle.2 = BorderColor::all(border_color);
        self
    }
}

impl EasyNode for EasyHorizontalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.0
    }
}

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

//>--------------------- HELPERS --------------------------

impl std::ops::Deref for EasyHorizontalLayoutStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl From<EasyHorizontalLayout> for (Node, BackgroundColor, BorderColor) {
    fn from(layout: EasyHorizontalLayout) -> (Node, BackgroundColor, BorderColor) {
       (layout.0, layout.1, layout.2)
    }
}

use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode, parts::{EasyStyle, EasyStyleExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyHorizontalLayout {
    pub node: Node,
    pub box_style: EasyStyle,
}

pub struct EasyHorizontalLayoutContainer {
    bundle: EasyHorizontalLayout,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyHorizontalLayoutStyle {
    pub node: Node,
    pub box_style: EasyStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyNode for EasyHorizontalLayoutContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
    }
}

impl EasyStyleExt for EasyHorizontalLayoutContainer {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.bundle.box_style }
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


//>--------------------- BUILDER API ---------------------

impl EasyHorizontalLayout {
    pub fn new() -> EasyHorizontalLayoutContainer {
        EasyHorizontalLayoutContainer {
            bundle: EasyHorizontalLayout {
                node: Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                box_style: EasyStyle::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyHorizontalLayout {
            node: Node::default(),
            box_style: EasyStyle::default(),
        }
    }
}

impl EasyHorizontalLayoutContainer {
    pub fn with_style(mut self, style: EasyHorizontalLayoutStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.box_style = style.box_style;
        self
    }
}

use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode, parts::{EasyStyle, EasyStyleExt}};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyButton {
    pub button: Button,
    pub node: Node,
    pub box_style: EasyStyle,
}

pub struct EasyButtonContainer {
    bundle: EasyButton,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyButtonStyle {
    pub node: Node,
    pub box_style: EasyStyle,
}

//>--------------------- ACCESSOR IMPL ---------------------

impl EasyStyleExt for EasyButtonContainer {
    fn easy_style_mut(&mut self) -> &mut EasyStyle { &mut self.bundle.box_style }
}

impl EasyNode for EasyButtonContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.node
    }
}

impl Container for EasyButtonContainer {
    fn take_bundle(&mut self) -> impl Bundle {
        std::mem::replace(&mut self.bundle, EasyButton::default_bundle())
    }
    fn take_children(&mut self) -> Vec<EasyElement> { std::mem::take(&mut self.children) }
    fn take_observers(&mut self) -> Vec<Observer> { std::mem::take(&mut self.observers) }
}
impl PushChild for EasyButtonContainer {
    fn push_child(&mut self, c: EasyElement) { self.children.push(c); }
}
impl PushObserver for EasyButtonContainer {
    fn push_observer(&mut self, o: Observer) { self.observers.push(o); }
}

//>--------------------- BUILDER API ---------------------

impl EasyButton {
    pub fn new() -> EasyButtonContainer {
         EasyButtonContainer {
            bundle: EasyButton {
                button: Button,
                node: Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                box_style: EasyStyle::default(),
            },
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyButton {
            button: Button,
            node: Node::default(),
            box_style: EasyStyle::default(),
        }
    }
}

impl EasyButtonContainer {
    pub fn with_style(mut self, style: EasyButtonStyle) -> Self {
        self.bundle.node = style.node;
        self.bundle.box_style.border_color = style.box_style.border_color;
        self.bundle.box_style.background_color = style.box_style.background_color;
        self.bundle.box_style.box_shadow = style.box_style.box_shadow;
        self
    }
}

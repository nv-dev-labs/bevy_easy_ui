use bevy::prelude::*;

use crate::core::{container::{Container, PushChild, PushObserver}, element::EasyElement, node::EasyNode};

//>--------------------- STRUCTURES ---------------------

#[derive(Bundle, Debug)]
pub struct EasyButton (
    Button,
    Node,
    BorderColor,
    BackgroundColor,
);

pub struct EasyButtonContainer {
    bundle: EasyButton,
    children: Vec<EasyElement>,
    observers: Vec<Observer>,
}

#[derive(Default, Debug)]
pub struct EasyButtonStyle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

//>--------------------- IMPLEMENTATION ---------------------

//? Implementation of the "builder API" part of EasyButton, which lets us build the declarative definition of the button (without spawning it)
impl EasyButton {
    pub fn new() -> EasyButtonContainer {
         EasyButtonContainer {
            bundle: EasyButton(Button, Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }, BorderColor::default(), BackgroundColor::default()),
            children: Vec::new(),
            observers: Vec::new(),
        }
    }

    fn default_bundle() -> Self {
        EasyButton(Button, Node::default(), BorderColor::default(), BackgroundColor::default())
    }
}

//? Style setters + EasyNode on the container so that we can chain
//? `.width(...)`, `.border_color(...)`, etc. directly after `EasyButton::new()`.
impl EasyButtonContainer {
    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.bundle.2 = BorderColor::all(border_color);
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.bundle.3 = BackgroundColor(background_color);
        self
    }

    pub fn with_style(mut self, style: EasyButtonStyle) -> Self {
        self.bundle.1 = style.node;
        self.bundle.2 = style.border_color;
        self.bundle.3 = style.background_color;
        self
    }
}

impl EasyNode for EasyButtonContainer {
    fn node_mut(&mut self) -> &mut Node {
        &mut self.bundle.1
    }
}

//? Implementation of the Container trait for EasyButtonContainer, which lets us treat it as a node of the UI tree with children and observers
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

//>--------------------- HELPERS ---------------------

impl std::ops::Deref for EasyButtonStyle {
    type Target = Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

// This allows us to convert an EasyButton into the actual components needed to spawn it in Bevy
impl From<EasyButton> for (Button, Node, BorderColor, BackgroundColor,) {
    fn from(button: EasyButton) -> (Button, Node, BorderColor, BackgroundColor) {
       (button.0, button.1, button.2, button.3)
    }
}
